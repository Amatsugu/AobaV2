using System.Security.Claims;
using AobaCore;
using AobaCore.Services;
using AobaServer;
using AobaServer.Auth;
using AobaServer.Middleware;
using AobaServer.Models;
using AobaServer.Services;
using AobaServer.Utils;
using Microsoft.AspNetCore.Authentication;
using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.AspNetCore.Http.Features;
using Microsoft.Extensions.Options;
using Microsoft.IdentityModel.Tokens;

using MongoDB.Driver;
using MongoDB.Driver.Core.Extensions.DiagnosticSources;

var builder = WebApplication.CreateBuilder(args);

builder.WebHost.ConfigureKestrel(o =>
{
	o.Limits.MaxRequestBodySize = null;
	o.ListenAnyIP(8081, lo =>
	{
		lo.Protocols = Microsoft.AspNetCore.Server.Kestrel.Core.HttpProtocols.Http1AndHttp2;
	});
});
var config = builder.Configuration;
// Add services to the container.
builder.Services.AddControllers(opt => opt.ModelBinderProviders.Add(new BsonIdModelBinderProvider()));

builder.Services.AddObservability(builder.Configuration);
builder.Services.AddGrpc();

//DB
var dbString = config["DB_STRING"] ?? throw new InvalidOperationException("DB_STRING is not set");
var dbName = config["DB_NAME"] ?? throw new InvalidOperationException("DB_NAME is not set");
Console.WriteLine($"Using db: '{dbName}'");
var settings = MongoClientSettings.FromConnectionString(dbString);
settings.ClusterConfigurator = cb => cb.Subscribe(new DiagnosticsActivityEventSubscriber());
var dbClient = new MongoClient(settings);
var db = dbClient.GetDatabase(dbName);

builder.Services.AddSingleton(dbClient);
builder.Services.AddSingleton<IMongoDatabase>(db);
builder.Services.AddSingleton<PasskeyCreationOptsCache>();
builder.Services.AddSingleton<PasskeyAssertionOptsCache>();

var authCfg = new AuthConfigService(db);
builder.Services.AddSingleton(authCfg);


var userAuthInfo = await authCfg.GetDefaultAuthInfoAsync();


var host = config["HOST"] ?? throw new InvalidOperationException("HOST is not set");
builder.Services.AddCors(o =>
{
	o.AddPolicy("AllowAll", p =>
	{
		p.AllowAnyOrigin();
		p.AllowAnyMethod();
		p.AllowAnyHeader();
	});
	o.AddPolicy("RPC", p =>
	{
		p.AllowAnyMethod();
		p.AllowAnyHeader();
		p.WithExposedHeaders("Grpc-Status", "Grpc-Message", "Grpc-Encoding", "Grpc-Accept-Encoding");
#if DEBUG
		p.AllowAnyOrigin();
#else
		p.WithOrigins(new Uri(host).GetLeftPart(UriPartial.Authority));
#endif
	});
});

builder.Services.AddAuthorizationBuilder()
	.AddPolicy(Consts.MetricsAuthPolicy, p =>
	{
		p.AddAuthenticationSchemes(Consts.MetricsAuthScheme)
			.RequireAuthenticatedUser()
			.RequireRole("metrics");
	});
builder.Services.AddRequestTimeouts(cfg =>
{
	cfg.AddPolicy("upload", TimeSpan.FromMinutes(10));
	cfg.AddPolicy("thumbnails", TimeSpan.FromMinutes(10));
});

var metricsAuthInfo = await authCfg.GetAuthInfoAsync("aoba", "metrics");
builder.Services.AddAuthentication(options =>
{
	options.DefaultAuthenticateScheme = JwtBearerDefaults.AuthenticationScheme;
	options.DefaultChallengeScheme = Consts.AobaChallengeScheme;
}).AddJwtBearer(JwtBearerDefaults.AuthenticationScheme, options => //Bearer auth
{
	options.TokenValidationParameters = new TokenValidationParameters
	{
		ValidateIssuerSigningKey = true,
		IssuerSigningKey = new SymmetricSecurityKey(userAuthInfo.SecureKey),
		ValidateIssuer = true,
		ValidIssuer = userAuthInfo.Issuer,
		ValidateAudience = true,
		ValidAudience = userAuthInfo.Audience,
		ValidateLifetime = true,
		ClockSkew = TimeSpan.FromMinutes(1),
	};
	options.Events = new JwtBearerEvents
	{
		OnTokenValidated = async ctx =>
		{
			var sessions = ctx.HttpContext.RequestServices.GetRequiredService<UserSessionsService>();
			if (ctx.Principal == null)
			{
				ctx.Fail("User is not validated");
				return;
			}
			var session = await sessions.VerifyTokenUserSessionAsync(ctx.Principal.GetId(), ctx.Principal.FindFirstValue("tokenId").ToObjectId());
			if (session == null)
			{
				ctx.Fail("Session Expired");
				return;
			}
		},
#if DEBUG //allow cookie based auth when in debug mode
		OnMessageReceived = ctx =>
		{
			if (string.IsNullOrWhiteSpace(ctx.Token))
				ctx.Token = ctx.Request.Cookies["token"];

			return Task.CompletedTask;
		},
#endif
	};
}).AddJwtBearer(Consts.MetricsAuthScheme, opts =>
{
	opts.TokenValidationParameters = new TokenValidationParameters
	{

		ValidateIssuerSigningKey = true,
		IssuerSigningKey = new SymmetricSecurityKey(metricsAuthInfo.SecureKey),
		ValidateIssuer = true,
		ValidIssuer = metricsAuthInfo.Issuer,
		ValidateAudience = true,
		ValidAudience = metricsAuthInfo.Audience,
		ValidateLifetime = false,
		ClockSkew = TimeSpan.FromMinutes(1)
	};

}).AddScheme<AuthenticationSchemeOptions, AobaAuthenticationHandler>(Consts.AobaChallengeScheme, null);


builder.Services.AddAoba().AddAobaS3();
builder.Services.AddFido2(opts =>
{
	opts.RPName = "Aoba";
	opts.RPID = new Uri(host).Host;
#if DEBUG
	opts.Origins = new HashSet<string> { "http://localhost:8080", "http://127.0.0.1:8080" };
#else
	opts.Origins = new HashSet<string> { host };
#endif
});
#if DEBUG
builder.Services.AddHostedService<DebugService>();
#endif
builder.Services.AddHostedService<RetroTaggerService>();
builder.Services.Configure<FormOptions>(opt =>
{
	opt.ValueLengthLimit = int.MaxValue;
	opt.MultipartBodyLengthLimit = long.MaxValue;
});

var app = builder.Build();

// Configure the HTTP request pipeline.
if (!app.Environment.IsDevelopment())
{
	app.UseExceptionHandler("/Home/Error");
	// The default HSTS value is 30 days. You may want to change this for production scenarios, see https://aka.ms/aspnetcore-hsts.
	app.UseHsts();
}

app.UseForwardedHeaders();
app.UseStaticFiles();
app.UseRouting();
app.UseGrpcWeb(new GrpcWebOptions { DefaultEnabled = true });
app.UseRequestTimeouts();

app.UseCors();


app.UseAuthentication();
app.UseAuthorization();

app.MapControllers();
app.MapGrpcService<AobaRpcService>()
	.RequireAuthorization()
	.RequireCors("RPC");
app.MapGrpcService<AccountRpcService>()
	.RequireAuthorization()
	.RequireCors("RPC");
app.MapGrpcService<MetricsRpcService>()
	.RequireAuthorization()
	.RequireCors("RPC");
app.MapGrpcService<AobaAuthService>()
	.AllowAnonymous()
	.RequireCors("RPC");
app.MapObservability();
app.MapFallbackToFile("index.html");

app.Run();

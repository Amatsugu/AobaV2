using AobaCore;

using AobaServer.Auth;
using AobaServer.Middleware;
using AobaServer.Models;
using AobaServer.Services;

using Microsoft.AspNetCore.Authentication;
using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.AspNetCore.Http.Features;
using Microsoft.IdentityModel.Tokens;

var builder = WebApplication.CreateBuilder(args);

builder.WebHost.ConfigureKestrel(o =>
{
#if !DEBUG
	o.ListenAnyIP(8081, lo =>
    {
        lo.Protocols = Microsoft.AspNetCore.Server.Kestrel.Core.HttpProtocols.Http2;
    });
	o.ListenAnyIP(8080, lo =>
    {
        lo.Protocols = Microsoft.AspNetCore.Server.Kestrel.Core.HttpProtocols.Http1AndHttp2;
    });
#endif
});
var config = builder.Configuration;
// Add services to the container.
builder.Services.AddControllers(opt => opt.ModelBinderProviders.Add(new BsonIdModelBinderProvider()));

builder.Services.AddObersability(builder.Configuration);
builder.Services.AddGrpc();

var authInfo = AuthInfo.LoadOrCreate("Auth.json", "aobaV2", "aoba");
builder.Services.AddSingleton(authInfo);
var signingKey = new SymmetricSecurityKey(authInfo.SecureKey);

var validationParams = new TokenValidationParameters
{
	ValidateIssuerSigningKey = true,
	IssuerSigningKey = signingKey,
	ValidateIssuer = true,
	ValidIssuer = authInfo.Issuer,
	ValidateAudience = true,
	ValidAudience = authInfo.Audience,
	ValidateLifetime = false,
	ClockSkew = TimeSpan.FromMinutes(1),
};

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
		p.AllowAnyOrigin();
	});
});

builder.Services.AddAuthentication(options =>
{
	options.DefaultAuthenticateScheme = JwtBearerDefaults.AuthenticationScheme;
	options.DefaultChallengeScheme = "Aoba";
}).AddJwtBearer(JwtBearerDefaults.AuthenticationScheme, options => //Bearer auth
{
	options.TokenValidationParameters = validationParams;
	options.TokenHandlers.Add(new MetricsTokenValidator(authInfo));
	options.Events = new JwtBearerEvents
	{
		OnMessageReceived = ctx => //Retreive token from cookie if not found in headers
		{
			if (string.IsNullOrWhiteSpace(ctx.Token))
				ctx.Token = ctx.Request.Headers.Authorization.FirstOrDefault()?.Replace("Bearer ", "");

#if DEBUG //allow cookie based auth when in debug mode
			if (string.IsNullOrWhiteSpace(ctx.Token))
				ctx.Token = ctx.Request.Cookies.FirstOrDefault(c => c.Key == "token").Value;
#endif

			return Task.CompletedTask;
		},
		OnAuthenticationFailed = ctx =>
		{
			ctx.Response.Cookies.Append("token", "", new CookieOptions
			{
				MaxAge = TimeSpan.Zero,
				Expires = DateTime.Now
			});
			ctx.Options.ForwardChallenge = "Aoba";

			return Task.CompletedTask;
		}
	};
}).AddScheme<AuthenticationSchemeOptions, AobaAuthenticationHandler>("Aoba", null);

var dbString = config["DB_STRING"];
builder.Services.AddAoba(dbString ?? "mongodb://localhost:27017");
builder.Services.Configure<FormOptions>(opt =>
{
	opt.ValueLengthLimit = int.MaxValue;
	opt.MultipartBodyLengthLimit = int.MaxValue;
});

var app = builder.Build();

// Configure the HTTP request pipeline.
if (!app.Environment.IsDevelopment())
{
	app.UseExceptionHandler("/Home/Error");
	// The default HSTS value is 30 days. You may want to change this for production scenarios, see https://aka.ms/aspnetcore-hsts.
	app.UseHsts();
}

app.UseGrpcWeb(new GrpcWebOptions { DefaultEnabled = true });
app.UseStaticFiles();
app.UseRouting();


app.UseCors();


app.UseAuthentication();
app.UseAuthorization();

app.MapControllers();
app.MapObserability();
app.MapGrpcService<AobaRpcService>()
	.RequireAuthorization()
	.RequireCors("RPC");
app.MapGrpcService<AobaAuthService>()
	.AllowAnonymous()
	.RequireCors("RPC");
app.MapFallbackToFile("index.html");

app.Run();

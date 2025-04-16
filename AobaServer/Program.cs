using AobaCore;

using AobaServer;
using AobaServer.Models;

using Microsoft.AspNetCore.Authentication;
using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.AspNetCore.Http.Features;
using Microsoft.IdentityModel.Tokens;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
builder.Services.AddControllers(opt => opt.ModelBinderProviders.Add(new BsonIdModelBinderProvider()));


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

builder.Services.AddAuthentication(options =>
{
	options.DefaultAuthenticateScheme = JwtBearerDefaults.AuthenticationScheme;
	options.DefaultChallengeScheme = "Aoba";
}).AddJwtBearer(JwtBearerDefaults.AuthenticationScheme, options => //Bearer auth
{
	options.TokenValidationParameters = validationParams;
	options.Events = new JwtBearerEvents
	{
		OnMessageReceived = ctx => //Retreive token from cookie if not found in headers
		{
			if (string.IsNullOrWhiteSpace(ctx.Token))
				ctx.Token = ctx.Request.Headers.Authorization.FirstOrDefault()?.Replace("Bearer ", "");
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

builder.Services.AddAoba();
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


app.UseHttpsRedirection();
app.UseRouting();



app.UseAuthentication();
app.UseAuthorization();


app.MapControllers();


app.Run();

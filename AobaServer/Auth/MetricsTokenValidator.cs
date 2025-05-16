using AobaServer.Models;

using Microsoft.IdentityModel.Tokens;

using System.IdentityModel.Tokens.Jwt;
using System.Security.Claims;

namespace AobaServer.Auth;

public class MetricsTokenValidator(AuthInfo authInfo) : JwtSecurityTokenHandler
{
	private readonly JwtSecurityTokenHandler _handler = new();
	public override Task<TokenValidationResult> ValidateTokenAsync(string token, TokenValidationParameters validationParameters)
	{
		try
		{
			var principal = _handler.ValidateToken(token, new TokenValidationParameters
			{
				ValidateIssuerSigningKey = true,
				IssuerSigningKey = new SymmetricSecurityKey(authInfo.SecureKey),
				ValidateIssuer = true,
				ValidIssuer = authInfo.Issuer,
				ValidateAudience = true,
				ValidAudience = "metrics",
				ValidateLifetime = false,
				ClockSkew = TimeSpan.FromMinutes(1)
			}, out var validatedToken);
			return Task.FromResult(new TokenValidationResult
			{
				IsValid = true,
				SecurityToken = validatedToken,
				ClaimsIdentity = new ClaimsIdentity(principal.Identity),
			});
		}
		catch (Exception e)
		{
			return Task.FromResult(new TokenValidationResult
			{
				IsValid = false,
				Exception = e
			});
		}
	}
}

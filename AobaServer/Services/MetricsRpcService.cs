using Aoba.RPC;
using Aoba.RPC.Auth;

using Google.Protobuf.WellKnownTypes;

using Grpc.Core;

using Microsoft.AspNetCore.Authorization;
using Microsoft.IdentityModel.Tokens;

using System.IdentityModel.Tokens.Jwt;

namespace AobaServer.Services;

public class MetricsRpcService(AuthConfigService authConfig): Aoba.RPC.Metrics.MetricsRpc.MetricsRpcBase
{
	[AllowAnonymous]
	public override async Task<Jwt> GetToken(Empty request, ServerCallContext context)
	{
		var authInfo = await authConfig.GetAuthInfoAsync("aoba", "metrics");
		var handler = new JwtSecurityTokenHandler();

		var jwt = handler.CreateEncodedJwt(new SecurityTokenDescriptor
		{
			Audience = authInfo.Audience,
			Issuer = authInfo.Issuer,
			IssuedAt = DateTime.UtcNow,
			SigningCredentials = new SigningCredentials(new SymmetricSecurityKey(authInfo.SecureKey), SecurityAlgorithms.HmacSha256)
		});

		return new Jwt { Token = jwt };
	}
}

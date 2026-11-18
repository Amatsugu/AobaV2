using AobaCore.Models;

using AobaServer.Models;

using Grpc.Core;

using Microsoft.IdentityModel.Tokens;

using MongoDB.Bson;
using MongoDB.Driver;

using System.IdentityModel.Tokens.Jwt;
using System.Security.Claims;

namespace AobaServer.Utils;

public static class Extensions
{
	public static ObjectId ToObjectId(this string? value)
	{
		if (value == null)
			return ObjectId.Empty;
		if (ObjectId.TryParse(value, out ObjectId result))
			return result;
		return ObjectId.Empty;
	}

	extension(User user)
	{
		public (string token, DateTime exp) GetToken(AuthInfo authInfo, ObjectId tokenId)
		{
			var handler = new JwtSecurityTokenHandler();
			var signCreds = new SigningCredentials(new SymmetricSecurityKey(authInfo.SecureKey), SecurityAlgorithms.HmacSha256);
			var identity = user.GetIdentity(tokenId);
			var exp = DateTime.UtcNow.AddDays(5);
			var token = handler.CreateEncodedJwt(authInfo.Issuer, authInfo.Audience, identity,
				 notBefore: DateTime.UtcNow, expires: exp, issuedAt: DateTime.UtcNow, signCreds);
			return (token, exp);
		}

	}

	extension(ClaimsPrincipal user)
	{
		public ObjectId GetId()
		{
			return user.FindFirstValue(ClaimTypes.NameIdentifier).ToObjectId();
		}
		public ObjectId GetTokenId()
		{
			return user.FindFirstValue(ClaimTypes.Sid).ToObjectId();
		}
	}

	extension(ServerCallContext context)
	{
		public ObjectId GetUserId()
		{
			return context.GetHttpContext().User.GetId();
		}

		public ObjectId GetTokenId()
		{
			return context.GetHttpContext().User.GetTokenId();
		}

		public string GetUserAgent()
		{
			return context.RequestHeaders.GetValue("User-Agent") ?? string.Empty;
		}
	}

	public static string GetUserAgent(this HttpContext context)
	{
		return context.Request.Headers.UserAgent.ToString();
	}
}

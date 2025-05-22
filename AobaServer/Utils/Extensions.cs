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
		if(value == null)
			return ObjectId.Empty;
		if(ObjectId.TryParse(value, out ObjectId result))
			return result;
		return ObjectId.Empty;
	}

	public static string GetToken(this User user, AuthInfo authInfo)
	{
		var handler = new JwtSecurityTokenHandler();
		var signCreds = new SigningCredentials(new SymmetricSecurityKey(authInfo.SecureKey), SecurityAlgorithms.HmacSha256);
		var identity = user.GetIdentity();
		var token = handler.CreateEncodedJwt(authInfo.Issuer, authInfo.Audience, identity, notBefore: DateTime.Now, expires: null, issuedAt: DateTime.Now, signCreds);
		return token;
	}


	public static ObjectId GetId(this ClaimsPrincipal user)
	{
		return user.FindFirstValue(ClaimTypes.NameIdentifier).ToObjectId();
	}

	public static ObjectId GetUserId(this ServerCallContext context)
	{
		return context.GetHttpContext().User.GetId();
	}
}

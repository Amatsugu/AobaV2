using Fido2NetLib.Objects;

using Microsoft.IdentityModel.Tokens;

using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;

using System;
using System.Security.Claims;
using System.Text;
using System.Threading.Tasks;

namespace AobaCore.Models;

public class User
{
	[BsonId]
	public ObjectId Id { get; set; }
	public required string Username { get; set; }
	public required string PasswordHash { get; set; }
	public required string Role { get; set; }
	public bool IsArgon { get; set; }
	public ObjectId[] ApiKeys { get; set; } = [];
	public List<ObjectId> RegTokens { get; set; } = [];
	public List<StoredCredential> Credentials { get; set; } = [];

	public ClaimsIdentity GetIdentity(ObjectId tokenId)
	{
		var id = new ClaimsIdentity(new[]
		{
			new Claim(ClaimTypes.NameIdentifier, Id.ToString()),
			new Claim(ClaimTypes.Name, Username),
			new Claim(ClaimTypes.Role, "user"),
			new Claim("tokenId", tokenId.ToString())
		});

		if (Role != null)
			id.AddClaim(new Claim(ClaimTypes.Role, Role));
		return id;
	}


	public List<PublicKeyCredentialDescriptor> GetCredentialDescriptors() => [.. Credentials.Select(c => c.Descriptor)];
}


public class UserSession
{
	[BsonId]
	public required ObjectId TokenId { get; set; }
	public required ObjectId UserId { get; set; }
	public required string UserAgent { get; set; }
	public required string Token { get; set; }
	public required DateTime DateCreated { get; set; }
	public required DateTime LastUsed { get; set; }
}
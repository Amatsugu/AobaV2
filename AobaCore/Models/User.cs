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

	public ClaimsIdentity GetIdentity()
	{
		var id = new ClaimsIdentity(new[]
		{
				new Claim(ClaimTypes.NameIdentifier, Id.ToString()),
				new Claim(ClaimTypes.Name, Username),
			});

		if (Role != null)
			id.AddClaim(new Claim(ClaimTypes.Role, Role));
		return id;
	}
}

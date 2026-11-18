using System.Security.Claims;
using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;

namespace AobaCore.Models;


public class AccessToken
{
	[BsonId]
	public ObjectId TokenId { get; set; }
	public required string Label { get; set; }
	public required string Token { get; set; }
	public DateTime IssuedDate { get; set; }
	public DateTime LastUsed { get; set; }
	public required string[] Roles { get; set; }
	public ObjectId UserId { get; set; }

	public ClaimsIdentity GetIdentity()
	{
		var id = new ClaimsIdentity(new[]
		{
			new Claim(ClaimTypes.NameIdentifier, UserId.ToString()),
			new Claim(ClaimTypes.Role, "api"),
			new Claim(ClaimTypes.Sid, TokenId.ToString())
		});

		foreach (var role in Roles)
			id.AddClaim(new Claim(ClaimTypes.Role, role));
		return id;
	}
}
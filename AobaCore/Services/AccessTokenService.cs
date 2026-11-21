using System.Security.Claims;
using AobaCore.Models;
using MongoDB.Bson;
using MongoDB.Driver;

namespace AobaCore.Services;


public class AccessTokenService(IMongoDatabase db)
{
	private readonly IMongoCollection<AccessToken> _accessTokens = db.GetCollection<AccessToken>(Collections.AccessTokens);

	public async Task<AccessToken> AddTokenAsync(ObjectId ownerId, string label, string[] roles, Func<ClaimsIdentity, string> tokenFactory, CancellationToken cancellationToken = default)
	{
		var tokenId = ObjectId.GenerateNewId();
		var token = new AccessToken
		{
			IssuedDate = DateTime.Now,
			Label = label,
			Roles = roles,
			Token = string.Empty,
			TokenId = tokenId,
			UserId = ownerId
		};
		token.Token = tokenFactory(token.GetIdentity());
		await _accessTokens.InsertOneAsync(token, null, cancellationToken);
		return token;
	}

	public async Task RevokeTokenAsync(ObjectId tokenId, ObjectId ownerId, CancellationToken cancellationToken = default)
	{
		await _accessTokens.DeleteOneAsync(t => t.TokenId == tokenId && t.UserId == ownerId, null, cancellationToken);
	}

	public async Task<AccessToken?> GetAccessTokenAsync(ObjectId tokenId, bool updateUseDate = false, CancellationToken cancellationToken = default)
	{
		if (updateUseDate)
		{
			var update = Builders<AccessToken>.Update.Set(t => t.LastUsed, DateTime.UtcNow);
			await _accessTokens.UpdateOneAsync(t => t.TokenId == tokenId, update, null, cancellationToken);
		}
		return await _accessTokens.Find(t => t.TokenId == tokenId).FirstOrDefaultAsync(cancellationToken);
	}

	public async Task<List<AccessToken>> GetAccessTokensAsync(ObjectId ownerId, CancellationToken cancellationToken = default)
	{
		return await _accessTokens.Find(t => t.UserId == ownerId).ToListAsync(cancellationToken);
	}
}
using Amazon.S3.Model;

using AobaCore.Models;

using MongoDB.Bson;
using MongoDB.Driver;

using UAParser;

namespace AobaCore.Services;

public class UserSessionsService(IMongoDatabase db)
{
	private readonly IMongoCollection<UserSession> _sessions = db.GetCollection<UserSession>("userSessions");


	public async Task<UserSession> CreateUserSessionAsync(ObjectId userId, string userAgent, Func<ObjectId, (string name, DateTime exp)> tokenFactory, CancellationToken cancellationToken = default)
	{
		var tokenId = ObjectId.GenerateNewId();
		var parser = Parser.GetDefault();
		var client = parser.Parse(userAgent);
		var (token, exp) = tokenFactory(tokenId);
		var session = new UserSession
		{
			TokenId = tokenId,
			UserId = userId,
			DateCreated = DateTime.UtcNow,
			LastUsed = DateTime.UtcNow,
			Expiration = exp,
			UserAgent = userAgent,
			Label = $"{client.OS.Family} {client.Device.Family}",
			Token = token
		};
		await _sessions.InsertOneAsync(session, options: null, cancellationToken);
		return session;
	}


	public async Task RevokeUserSessionAsync(ObjectId userId, ObjectId tokenId, CancellationToken cancellationToken = default)
	{
		await _sessions.DeleteOneAsync(s => s.UserId == userId && s.TokenId == tokenId, cancellationToken);
	}

	public async Task<List<UserSession>> GetUserSessionsAsync(ObjectId userId, CancellationToken cancellationToken = default)
	{
		return await _sessions.Find(s => s.UserId == userId).ToListAsync(cancellationToken);
	}

	public async Task<UserSession?> GetUserSessionAsync(ObjectId tokenId, CancellationToken cancellationToken = default)
	{
		return await _sessions.Find(s => s.TokenId == tokenId).FirstOrDefaultAsync(cancellationToken);
	}

	public async Task<UserSession?> RefreshSession(ObjectId userId, ObjectId tokenId, Func<ObjectId, (string name, DateTime exp)> tokenFactory, CancellationToken cancellationToken = default)
	{
		var session = await _sessions.Find(s => s.UserId == userId).FirstOrDefaultAsync(cancellationToken);
		if (session == null || session.Expiration < DateTime.UtcNow)
			return null;
		var (token, exp) = tokenFactory(tokenId);
		session.Token = token;
		session.Expiration = exp;
		session.LastUsed = DateTime.UtcNow;
		var update = Builders<UserSession>.Update
			.Set(s => s.Token, token)
			.Set(s => s.Expiration, exp)
			.Set(s => s.LastUsed, session.LastUsed);
		await _sessions.UpdateOneAsync(s => s.TokenId == tokenId, update, null, cancellationToken);
		await _sessions.DeleteManyAsync(s => s.UserId == userId && s.Expiration <= DateTime.UtcNow, null, cancellationToken);
		return session;
	}


	public async Task UpdateTokenAsync(ObjectId tokenId, string token, DateTime expiration, CancellationToken cancellationToken = default)
	{
		var update = Builders<UserSession>.Update
			.Set(s => s.Token, token)
			.Set(s => s.Expiration, expiration)
			.Set(s => s.LastUsed, DateTime.UtcNow);
		await _sessions.UpdateOneAsync(s => s.TokenId == tokenId, update, null, cancellationToken);
	}

	public async Task UpdateUseDateAsync(ObjectId tokenId, CancellationToken cancellationToken = default)
	{
		var update = Builders<UserSession>.Update
			.Set(s => s.LastUsed, DateTime.UtcNow);
		await _sessions.UpdateOneAsync(s => s.TokenId == tokenId, update, null, cancellationToken);
	}
}
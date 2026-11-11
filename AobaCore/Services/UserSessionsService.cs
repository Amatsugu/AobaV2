using AobaCore.Models;
using MongoDB.Bson;
using MongoDB.Driver;

namespace AobaCore.Services;

public class UserSessionsService(IMongoDatabase db)
{
	private readonly IMongoCollection<UserSession> _sessions = db.GetCollection<UserSession>("userSessions");


	public async Task<UserSession> CreateUserSessionAsync(ObjectId userId, string userAgent, Func<ObjectId, string> tokenFactory, CancellationToken cancellationToken = default)
	{
		var tokenId = ObjectId.GenerateNewId();
		var session = new UserSession
		{
			TokenId = tokenId,
			UserId = userId,
			DateCreated = DateTime.Now,
			LastUsed = DateTime.Now,
			UserAgent = userAgent,
			Token = tokenFactory(tokenId)
		};
		await _sessions.InsertOneAsync(session, options: null, cancellationToken);
		return session;
	}


	public async Task RevokeUserSessionAsync(ObjectId userId, ObjectId tokenId, CancellationToken cancellationToken = default)
	{
		await _sessions.DeleteOneAsync(s => s.UserId == userId && s.TokenId == tokenId, cancellationToken);
	}

	public async Task<UserSession?> VerifyTokenUserSessionAsync(ObjectId userId, ObjectId tokenId, CancellationToken cancellationToken = default)
	{
		return await _sessions.Find(s => s.UserId == userId && s.TokenId == tokenId).FirstOrDefaultAsync(cancellationToken);
	}
}
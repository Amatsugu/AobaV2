using AobaServer.Models;

using MongoDB.Driver;

namespace AobaServer.Services;

public class AuthConfigService(IMongoDatabase db)
{
	public IMongoCollection<AuthInfo> _authInfo = db.GetCollection<AuthInfo>("auth_config");

	public async Task<AuthInfo> GetAuthInfoAsync(string issuer, string audience)
	{
		var info = await _authInfo.Find("{}").FirstOrDefaultAsync();
		if(info != null)
			return info;

		info = AuthInfo.Create(issuer, audience);
		await _authInfo.InsertOneAsync(info);
		return info;
	}

	public Task<AuthInfo> GetDefaultAuthInfoAsync()
	{
		return GetAuthInfoAsync("aobaV2", "aoba");
	}
}

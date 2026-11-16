using Amazon.Auth.AccessControlPolicy;

using AobaCore.Models;
using AobaCore.Services;

using AobaServer.Utils;

using MongoDB.Bson;

using System.Security.Claims;

namespace AobaServer.Services;

public class LoginService(AccountsService accountsService, UserSessionsService sessionsService, AuthConfigService authConfig)
{

	public async Task<UserSession?> VerifyUserSessionTokenAsync(ClaimsPrincipal principal, CancellationToken cancellationToken = default)
	{
		var (user, session) = await LoadUserAndSessionAsync(principal, cancellationToken);
		if (user == null || session == null)
			return null;
		return await RefreshUserSessionAsync(user, session, cancellationToken);
	}

	public async Task<UserSession?> RefreshUserSessionAsync(ClaimsPrincipal principal, CancellationToken cancellationToken = default)
	{
		var (user, session) = await LoadUserAndSessionAsync(principal, cancellationToken);
		if (user == null || session == null)
			return null;
		return await RefreshUserSessionAsync(user, session, cancellationToken);
	}

	public async Task<UserSession?> RefreshUserSessionAsync(User user, UserSession session, CancellationToken cancellationToken = default)
	{
		var authInfo = await authConfig.GetDefaultAuthInfoAsync();
		var timeLeft = DateTime.UtcNow - session.Expiration;
		if (timeLeft.Days < 1)
		{
			(session.Token, session.Expiration) = user.GetToken(authInfo, session.TokenId);
			await sessionsService.UpdateTokenAsync(session.TokenId, session.Token, session.Expiration, cancellationToken);
			session.LastUsed = DateTime.UtcNow;
		}
		else
			await sessionsService.UpdateUseDateAsync(session.TokenId, cancellationToken);
		return session;
	}

	private async Task<(User? user, UserSession? token)> LoadUserAndSessionAsync(ClaimsPrincipal principal, CancellationToken cancellationToken = default)
	{
		var userId = principal.GetId();
		var tokenId = principal.GetTokenId();
		var user = await accountsService.GetUserAsync(userId, cancellationToken);
		var session = await sessionsService.GetUserSessionAsync(tokenId, cancellationToken);
		return (user, session);
	}
}

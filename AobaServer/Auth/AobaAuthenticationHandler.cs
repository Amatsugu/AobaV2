using Microsoft.AspNetCore.Authentication;
using Microsoft.Extensions.Options;

using System.Text.Encodings.Web;

namespace AobaServer.Auth;

internal class AobaAuthenticationHandler(IOptionsMonitor<AuthenticationSchemeOptions> options, ILoggerFactory logger, UrlEncoder encoder) : AuthenticationHandler<AuthenticationSchemeOptions>(options, logger, encoder)
{
	protected override Task<AuthenticateResult> HandleAuthenticateAsync()
	{
		throw new NotImplementedException();
	}

	protected override Task HandleChallengeAsync(AuthenticationProperties properties)
	{
		Response.StatusCode = StatusCodes.Status401Unauthorized;
		Response.BodyWriter.Complete();
		return Task.CompletedTask;
	}

	protected override Task HandleForbiddenAsync(AuthenticationProperties properties)
	{
		Response.StatusCode = StatusCodes.Status403Forbidden;
		Response.BodyWriter.Complete();
		return Task.CompletedTask;
	}
}
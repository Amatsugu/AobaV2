using AobaCore.Services;

using AobaServer.Models;
using AobaServer.Utils;

using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;

using System.Net;

namespace AobaServer.Controllers;



//allow login via http during debug testing
#if DEBUG
[AllowAnonymous]
[Route("auth")]
public class AuthController(AccountsService accountsService, AuthInfo authInfo) : Controller
{
	[HttpPost("login")]
	public async Task<IActionResult> Login([FromForm] string username, [FromForm] string password, CancellationToken cancellationToken)
	{
		var user = await accountsService.VerifyLoginAsync(username, password, cancellationToken);

		if (user == null)
			return Problem("Invalid login Credentials", statusCode: StatusCodes.Status400BadRequest);
		Response.Cookies.Append("token", user.GetToken(authInfo), new CookieOptions
		{
			IsEssential = true,
			SameSite = SameSiteMode.Strict,
			Secure = true,
		});
		return Ok();
	}
}
#endif
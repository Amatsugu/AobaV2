using Microsoft.AspNetCore.Mvc;

namespace AobaServer.Controllers.Api;

[Route("/api/auth")]
public class AuthApi : ControllerBase
{
	[HttpGet("login")]
	public async Task<IActionResult> LoginAsync()
	{
		throw new NotImplementedException();
	}

	[HttpGet("register")]
	public async Task<IActionResult> RegisterAsync()
	{
		throw new NotImplementedException();
	}
}

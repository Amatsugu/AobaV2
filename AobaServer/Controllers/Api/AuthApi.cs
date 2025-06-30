using Microsoft.AspNetCore.Mvc;

namespace AobaServer.Controllers.Api;

[Route("/api/auth")]
public class AuthApi : ControllerBase
{
	[HttpGet("login")]
	public Task<IActionResult> LoginAsync()
	{
		throw new NotImplementedException();
	}

	[HttpGet("register")]
	public Task<IActionResult> RegisterAsync()
	{
		throw new NotImplementedException();
	}
}

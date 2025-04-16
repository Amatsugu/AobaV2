using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;

namespace AobaServer.Controllers;

[AllowAnonymous]
[Route("auth")]
public class AuthController : Controller
{
	[HttpGet("login")]
	public IActionResult Login([FromQuery] string returnUrl)
	{
		ViewData["returnUrl"] = returnUrl;
		return View();
	}

	[HttpGet("register/{token}")]
	public IActionResult Register(string token)
	{

		return View(token);
	}
}

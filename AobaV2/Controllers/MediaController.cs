using Microsoft.AspNetCore.Mvc;

namespace AobaV2.Controllers;
public class MediaController : Controller
{
	public IActionResult Index()
	{
		return View();
	}
}

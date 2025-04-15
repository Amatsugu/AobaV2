using AobaCore;

using Microsoft.AspNetCore.Mvc;

using MongoDB.Bson;
using MongoDB.Driver;

namespace AobaV2.Controllers;

[Route("/m")]
public class MediaController(MediaService media) : Controller
{
	[HttpGet("{id}")]
	public IActionResult Media(ObjectId id)
	{
		return View();
	}

	[HttpGet("/i/{id}/{*rest}")]
	public async Task<IActionResult> LegacyRedirectAsync(ObjectId id, string rest, [FromServices] AobaService aoba)
	{
		var media = await aoba.GetMediaAsync(id);
		if (media == null)
			return NotFound();
		return LocalRedirectPermanent($"/m/{media.Id}/{rest}");
	}
}

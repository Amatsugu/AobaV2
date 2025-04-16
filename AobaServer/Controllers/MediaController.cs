using AobaCore;

using HeyRed.Mime;

using Microsoft.AspNetCore.Mvc;

using MongoDB.Bson;
using MongoDB.Driver;

namespace AobaServer.Controllers;

[Route("/m")]
public class MediaController(MediaService mediaService, ILogger<MediaController> logger) : Controller
{
	[HttpGet("{id}")]
	[ResponseCache(Duration = int.MaxValue)]
	public async Task<IActionResult> MediaAsync(ObjectId id)
	{
		var file = await mediaService.GetMediaStreamAsync(id);
		if (file.HasError)
		{
			logger.LogError(file.Error.Exception, "Failed to load media stream");
			return NotFound();
		}
		var mime = MimeTypesMap.GetMimeType(file.Value.FileInfo.Filename);
		return File(file, mime, true);
	}

	/// <summary>
	/// Redirect legacy media urls to the new url
	/// </summary>
	/// <param name="id"></param>
	/// <param name="rest"></param>
	/// <param name="aoba"></param>
	/// <returns></returns>
	[HttpGet("/i/{id}/{*rest}")]
	public async Task<IActionResult> LegacyRedirectAsync(ObjectId id, string rest, [FromServices] AobaService aoba)
	{
		var media = await aoba.GetMediaAsync(id);
		if (media == null)
			return NotFound();
		return LocalRedirectPermanent($"/m/{media.MediaId}/{rest}");
	}
}

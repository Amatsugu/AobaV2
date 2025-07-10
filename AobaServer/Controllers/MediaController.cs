using AobaCore.Models;
using AobaCore.Services;

using HeyRed.Mime;

using Microsoft.AspNetCore.Mvc;

using MongoDB.Bson;
using MongoDB.Driver;

namespace AobaServer.Controllers;

[Route("/m")]
public class MediaController(AobaService aobaService, ILogger<MediaController> logger) : Controller
{
	[HttpGet("{id}")]
	[ResponseCache(Duration = int.MaxValue)]
	public async Task<IActionResult> MediaAsync(ObjectId id, [FromServices] MongoClient client, CancellationToken cancellationToken)
	{
		var file = await aobaService.GetFileStreamAsync(id, seekable: true, cancellationToken: cancellationToken);
		if (file.HasError)
		{
			logger.LogError(file.Error.Exception, "Failed to load media stream");
			return NotFound();
		}
		var mime = MimeTypesMap.GetMimeType(file.Value.FileInfo.Filename);
		_ = aobaService.IncrementViewCountAsync(id, cancellationToken);
		return File(file, mime, true);
	}

	/// <summary>
	/// Redirect legacy media urls to the new url
	/// </summary>
	/// <param name="legacyId"></param>
	/// <param name="rest"></param>
	/// <param name="aoba"></param>
	/// <returns></returns>
	[HttpGet("/i/{legacyId}/{*rest}")]
	public async Task<IActionResult> LegacyRedirectAsync(ObjectId legacyId, string rest, CancellationToken cancellationToken)
	{
		var media = await aobaService.GetMediaFromLegacyIdAsync(legacyId, cancellationToken);
		if (media == null)
			return NotFound();
		return LocalRedirectPermanent($"/m/{media.MediaId}/{rest}");
	}

	[HttpGet("{id}/thumb")]
	[ResponseCache(Duration = int.MaxValue)]
	public async Task<IActionResult> ThumbAsync(ObjectId id, [FromServices] ThumbnailService thumbnailService, [FromQuery] ThumbnailSize size = ThumbnailSize.Medium, CancellationToken cancellationToken = default)
	{
		var thumb = await thumbnailService.GetOrCreateThumbnailAsync(id, size, cancellationToken);
		if (thumb.HasError)
		{
			logger.LogError("Failed to generate thumbnail: {}", thumb.Error);
			return DefaultThumbnailAsync();
		}
		return File(thumb, "image/webp", true);
	}

	[HttpGet("/t/{id}")]
	public async Task<IActionResult> ThumbAsync(ObjectId id, [FromServices] ThumbnailService thumbnailService, CancellationToken cancellationToken = default)
	{
		var thumb = await thumbnailService.GetThumbnailByFileIdAsync(id, cancellationToken);
		if(thumb == null) 
			return NotFound();
		return File(thumb, "image/webp", true);
	}

	[NonAction]
	private IActionResult DefaultThumbnailAsync()
	{
		return NoContent();
	}
}

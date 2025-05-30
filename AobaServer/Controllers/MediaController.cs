﻿using AobaCore.Services;

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
		var file = await aobaService.GetFileStreamAsync(id, cancellationToken: cancellationToken);
		if (file.HasError)
		{
			logger.LogError(file.Error.Exception, "Failed to load media stream");
			return NotFound();
		}
		var mime = MimeTypesMap.GetMimeType(file.Value.FileInfo.Filename);
		_ = aobaService.IncrementFileViewCountAsync(id, cancellationToken);
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
	public async Task<IActionResult> LegacyRedirectAsync(ObjectId id, string rest, CancellationToken cancellationToken)
	{
		var media = await aobaService.GetMediaAsync(id, cancellationToken);
		if (media == null)
			return NotFound();
		return LocalRedirectPermanent($"/m/{media.MediaId}/{rest}");
	}

	[HttpGet("thumb/{id}")]
	public async Task<IActionResult> ThumbAsync(ObjectId id, [FromServices] ThumbnailService thumbnailService, CancellationToken cancellationToken)
	{
		var thumb = await thumbnailService.GetThumbnailFromFileAsync(id, cancellationToken);
		if (thumb == null)
			return NotFound();
		return File(thumb, "image/webp", true);
	}
}

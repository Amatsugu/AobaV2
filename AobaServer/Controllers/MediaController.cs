using AobaCore.Models;
using AobaCore.Services;

using Flurl;

using HeyRed.Mime;

using Microsoft.AspNetCore.Cors;
using Microsoft.AspNetCore.Http.Timeouts;
using Microsoft.AspNetCore.Mvc;

using MongoDB.Bson;
using MongoDB.Driver;

namespace AobaServer.Controllers;

[Route("/m")]
[EnableCors(PolicyName = "AllowAll")]
public class MediaController(AobaService aobaService, HostInfo hostInfo, ILogger<MediaController> logger) : Controller
{
	[HttpGet("{id}")]
	[ResponseCache(Duration = int.MaxValue)]
	public async Task<IActionResult> MediaAsync(ObjectId id, CancellationToken cancellationToken)
	{
		var media = await aobaService.GetMediaAsync(id, cancellationToken);
		if (media == null)
		{
			return NotFound();
		}
		else
		{
			if (media.Cdn != null)
			{
				_ = aobaService.IncrementViewCountAsync(id, cancellationToken);
				return Redirect(hostInfo.CdnHost.AppendPathSegment(media.Cdn.Url));
			}
			else
			{
				return NotFound();
			}
		}
	}

	/// <summary>
	/// Redirect legacy media urls to the new url
	/// </summary>
	/// <param name="legacyId"></param>
	/// <param name="rest"></param>
	/// <returns></returns>
	[HttpGet("/i/{legacyId}/{*rest}")]
	public async Task<IActionResult> LegacyRedirectAsync(ObjectId legacyId, CancellationToken cancellationToken)
	{
		var media = await aobaService.GetMediaFromLegacyIdAsync(legacyId, cancellationToken);
		if (media == null)
			return NotFound();
		return LocalRedirectPermanent($"/m/{media.MediaId}/");
	}

	[HttpGet("{id}/thumb")]
	[ResponseCache(Duration = int.MaxValue)]
	[RequestTimeout("thumbnails")]
	public async Task<IActionResult> ThumbAsync(ObjectId id, [FromServices] ThumbnailRequestService thumbnailService, [FromQuery] ThumbnailSize size = ThumbnailSize.Medium, CancellationToken cancellationToken = default)
	{
		var thumb = await thumbnailService.GetOrCreateThumbnailAsync(id, size, cancellationToken);
		if (thumb == null)
			return DefaultThumbnailAsync();
		return RedirectPermanent(thumb);
	}

	[NonAction]
	private NoContentResult DefaultThumbnailAsync()
	{
		return NoContent();
	}
}
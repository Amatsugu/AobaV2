using AobaCore.Models;
using AobaCore.Services;

using AobaServer.Utils;

using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Http.Timeouts;
using Microsoft.AspNetCore.Mvc;

using MongoDB.Bson;

namespace AobaServer.Controllers.Api;

[ApiController, Authorize]
[Route("/api/media")]
[RequestTimeout("upload")]
public class MediaApi(AobaService aoba, HostInfo hostInfo) : ControllerBase
{
	[HttpPost("upload")]
	public async Task<IActionResult> UploadAsync([FromForm] IFormFile file, CancellationToken cancellationToken)
	{
		//todo: switch to s3
		var media = await aoba.UploadFileAsync(file.OpenReadStream(), file.FileName, User.GetId(), cancellationToken);

		if (media.HasError)
			return Problem(detail: media.Error.Message, statusCode: StatusCodes.Status400BadRequest);


		return Ok(new
		{
			media = media.Value,
			url = media.Value.GetMediaUrl(hostInfo)
		});
	}
}

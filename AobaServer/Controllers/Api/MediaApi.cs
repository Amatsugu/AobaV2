using AobaCore.Models;
using AobaCore.Services;

using AobaServer.Utils;

using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;

using MongoDB.Bson;

namespace AobaServer.Controllers.Api;

[ApiController, Authorize]
[Route("/api/media")]
public class MediaApi(AobaService aoba) : ControllerBase
{
	[HttpPost("upload")]
	public async Task<IActionResult> UploadAsync([FromForm] IFormFile file, CancellationToken cancellationToken)
	{
		var media = await aoba.UploadFileAsync(file.OpenReadStream(), file.FileName, User.GetId(), cancellationToken);

		if (media.HasError)
			return Problem(detail: media.Error.Message, statusCode: StatusCodes.Status400BadRequest);

		return Ok(new
		{
			media = media.Value,
			url = media.Value.GetMediaUrl()
		});
	}

	[HttpDelete("{id}")]
	public async Task<IActionResult> Delete(ObjectId id, CancellationToken cancellationToken)
	{
		await aoba.DeleteFileAsync(id, cancellationToken);
		return Ok();
	}
}

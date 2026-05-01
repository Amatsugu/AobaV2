
using AobaCore.Models;
using AobaCore.Services;

namespace AobaServer.Services;

public class DebugService(AobaService aobaService, ThumbnailService thumbnailService) : BackgroundService
{
	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
		var mediaItems = await aobaService.FindMediaWithExtAsync(".ogg", stoppingToken);
		foreach (var item in mediaItems)
		{
			if(item.MediaType != MediaType.Audio)
				await aobaService.SetMediaTypeAsync(item.MediaId, MediaType.Audio);
		}
	}
}

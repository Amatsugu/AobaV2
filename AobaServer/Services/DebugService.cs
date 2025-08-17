
using AobaCore.Services;

namespace AobaServer.Services;

public class DebugService(AobaService aobaService, ThumbnailService thumbnailService) : BackgroundService
{
	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
		var mediaItems = await aobaService.FindMediaWithExtAsync(".avif", stoppingToken);
		foreach (var item in mediaItems)
		{
			foreach (var size in item.Thumbnails.Keys)
			{
				await thumbnailService.DeleteThumbnailAsync(item.MediaId, size);
			}
		}
	}
}

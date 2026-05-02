
using AobaCore.Models;
using AobaCore.Services;

namespace AobaServer.Services;

public class DebugService(AobaService aobaService, ThumbnailService thumbnailService) : BackgroundService
{
	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
		//todo: clean up orphaned thumbnails
	}
}

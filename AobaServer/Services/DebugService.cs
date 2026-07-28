using AobaCore.Models;
using AobaCore.Services;

using MongoDB.Driver;

namespace AobaServer.Services;

public class DebugService(IMongoDatabase db, ThumbnailService thumbnailService) : BackgroundService
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>("media");

	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
		var files = await _media.Find(f => f.MediaType == MediaType.Code || f.MediaType == MediaType.Text).ToListAsync(stoppingToken);

		foreach (var item in files)
		{
			await thumbnailService.DeleteThumbnailAsync(item.MediaId, ThumbnailSize.Medium, stoppingToken);
		}

		Console.WriteLine("Thumbnails deleted");
	}
}

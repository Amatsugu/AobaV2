using AobaCore.Models;
using AobaCore.Services;

using MongoDB.Driver;

namespace AobaServer.Services;

public class DebugService(AobaService aobaService, IMongoDatabase db, S3MediaService s3) : BackgroundService
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>("media");

	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
		var items = await _media.Find(Builders<Media>.Filter.Exists(m => m.Cdn, true)).ToListAsync();
		foreach (var item in items) 
		{
			if(item.Cdn == null) continue;
			if (item.Cdn.Url.StartsWith('/'))
				item.Cdn.Url = item.Cdn.Url[1..];
			
			var update = Builders<Media>.Update.Set(m => m.Cdn!.Url, item.Cdn.Url);
			foreach (var (size, url) in item.Cdn.ThumbnailUrls)
			{
				if (url.StartsWith('/'))
				{
					update = update.Set(m => m.Cdn!.ThumbnailUrls[size], url[1..]);
				}
			}
			await _media.UpdateOneAsync(m => m.MediaId == item.MediaId, update, null, stoppingToken);
		}
		Console.WriteLine("Cleanup complete");
	}
}
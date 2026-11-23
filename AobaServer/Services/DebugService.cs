#if DEBUG
using System.Text.Json;
using AobaCore.Models;
using AobaCore.Services;

using AobaServer.Utils;

using MongoDB.Bson;
using MongoDB.Driver;

namespace AobaServer.Services;

public class DebugService(S3MediaService s3, AobaService aoba, ThumbnailRequestService requestService, IMongoDatabase db) : BackgroundService
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>(Collections.Media);
	private readonly IMongoCollection<ThumbnailRequest> _req = db.GetCollection<ThumbnailRequest>(Collections.ThumbnailRequests);
	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
		//var filter = Builders<Media>.Filter.Exists(m => m.Cdn!.ThumbnailUrls[ThumbnailSize.ResponsiveMedium]);
		//var items = await _media.Find(filter).ToCursorAsync(stoppingToken);
		//await items.ForEachAsync(async item =>
		//{
		//	if(item.Cdn!.ThumbnailUrls.TryGetValue(ThumbnailSize.ResponsiveMedium, out var url))
		//	{
		//		if(!await s3.FileExistsAsync(url, stoppingToken))
		//			await requestService.DeleteThumbnailAsync(item.MediaId, ThumbnailSize.ResponsiveMedium);
		//	}
		//});

	}
}

#endif
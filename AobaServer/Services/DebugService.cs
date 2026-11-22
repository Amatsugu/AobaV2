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
		await requestService.DeleteThumbnailAsync(ObjectId.Parse("6a8ce8b794719194c4d3fd51"), ThumbnailSize.ResponsiveMedium);

	}
}

#endif
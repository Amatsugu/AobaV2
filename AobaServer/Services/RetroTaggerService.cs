using AobaCore.Models;
using AobaCore.Services;

using MongoDB.Driver;

namespace AobaServer.Services;

public class RetroTaggerService(IMongoDatabase db, AutoTagger tagger, ILogger<RetroTaggerService> logger) : BackgroundService
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>("media");

	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
		await _media.Find("{}").ForEachAsync(async item =>
		{
			var tags = await tagger.GetTagsAsync(item.Filename);
			if (!tags.Any())
				return;
			var combined = item.Tags.Concat(tags).Distinct().ToArray();
			if (combined.Length == item.Tags.Length)
				return;
			var update = Builders<Media>.Update.Set(m => m.Tags, combined);
			await _media.UpdateOneAsync(m => m.MediaId == item.MediaId, update, null, stoppingToken);
		}, stoppingToken);
		logger.LogInformation("Retro Tagging completed");
	}
}
using AobaCore.Models;
using AobaCore.Services;

using MongoDB.Driver;

namespace AobaServer.Services;

public class RetroTaggerService(IMongoDatabase db, AutoTagger tagger, ILogger<RetroTaggerService> logger) : BackgroundService
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>("media");

	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
		var items = await _media.Find("{}").ToListAsync(stoppingToken);
		foreach (var item in items)
		{
			var tags = await tagger.GetTagsAsync(item.Filename);
			if (!tags.Any())
				continue;
			var combined = item.Tags.Concat(tags).Distinct().ToArray();
			if (combined.Length == item.Tags.Length)
				continue;
			var update = Builders<Media>.Update.Set(m => m.Tags, combined);
			await _media.UpdateOneAsync(m => m.MediaId == item.MediaId, update, null, stoppingToken);
		}
		logger.LogInformation("Retro Tagging completed");
	}
}
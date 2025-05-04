using AobaCore.Models;

using Microsoft.Extensions.Hosting;

using MongoDB.Driver;

namespace AobaCore;

public class AobaIndexCreationService(IMongoDatabase db): BackgroundService
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>("media");

	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
		var textKeys = Builders<Media>.IndexKeys
			.Text(m => m.Filename);

		var textModel = new CreateIndexModel<Media>(textKeys, new CreateIndexOptions
		{
			Name = "Text",
			Background = true
		});

		await _media.EnsureIndexAsync(textModel);
	}
}
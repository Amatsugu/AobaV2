using AobaCore.Models;

using Microsoft.Extensions.Hosting;

using MongoDB.Bson;
using MongoDB.Bson.Serialization;
using MongoDB.Bson.Serialization.Serializers;
using MongoDB.Driver;

namespace AobaCore.Services;

public class AobaIndexCreationService(IMongoDatabase db): BackgroundService
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>("media");

	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
		BsonSerializer.RegisterSerializer(new EnumSerializer<ThumbnailSize>(BsonType.String));

		var mediaId = Builders<Media>.IndexKeys.Ascending(m => m.MediaId);

		var mediaIdModel = new CreateIndexModel<Media>(mediaId, new CreateIndexOptions
		{
			Name = "Media",
			Unique = true,
			Background = true
		});

		var textKeys = Builders<Media>.IndexKeys
			.Text(m => m.Filename)
			.Text(m => m.Ext)
			.Text(m => m.Tags);

		var textModel = new CreateIndexModel<Media>(textKeys, new CreateIndexOptions
		{
			Name = "Text",
			Background = true
		});

		await _media.EnsureIndexAsync(mediaIdModel);
		await _media.EnsureIndexAsync(textModel);
	}
}
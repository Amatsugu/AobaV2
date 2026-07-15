using AobaCore.Models;

using Microsoft.Extensions.Hosting;

using MongoDB.Driver;
using MongoDB.Driver.GridFS;

using System;
using System.Collections.Generic;
using System.Text;

namespace AobaCore.Services;

public class S3MigrationService(IMongoDatabase db, S3MediaService s3) : BackgroundService
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>("media");
	private readonly GridFSBucket _gridFs = new(db);
	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
		var toMigrate = await _media.Find(Builders<Media>.Filter.Exists(m => m.Cdn, false)).ToListAsync();
		foreach (var item in toMigrate)
		{
			await MigrateMediaItem(item);
		}
	}

	private async Task MigrateMediaItem(Media media)
	{

		await MigrateThumbnails(media);
	}

	private async Task MigrateThumbnails(Media media)
	{

	}
}

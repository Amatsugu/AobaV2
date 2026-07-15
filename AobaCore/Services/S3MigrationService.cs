using AobaCore.Models;

using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;

using MongoDB.Bson;
using MongoDB.Driver;
using MongoDB.Driver.GridFS;

namespace AobaCore.Services;

public partial class S3MigrationService(IMongoDatabase db, S3MediaService s3, ILogger<S3MigrationService> logger) : BackgroundService
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>("media");
	private readonly GridFSBucket _gridFs = new(db);
	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
		var toMigrate = await _media.Find(Builders<Media>.Filter.Exists(m => m.Cdn, false)).ToListAsync();
		var prog = 0;
		foreach (var item in toMigrate)
		{
			prog++;
			try
			{
				await MigrateMediaItem(item);
			}
			catch(Exception ex) 
			{
				logger.LogError(ex, "Failed to migrate item: {}", item.MediaId);
			}
			if (prog % 100 == 0)
			{
#pragma warning disable CA1873 // Avoid potentially expensive logging
				logger.LogInformation("[{prog}/{total}] Migrating...", prog, toMigrate.Count);
#pragma warning restore CA1873 // Avoid potentially expensive logging
			}
		}
		logger.LogInformation("Migration Finished");
	}

	private async Task MigrateMediaItem(Media media)
	{
		using var file = await _gridFs.OpenDownloadStreamAsync(media.MediaId);
		var result = await s3.UploadFileAsync(media.GetS3Filename(), media.GetMimeType(), file);
		if (result.HasError)
		{
			logger.LogError("Failed to upload to s3 {id} - {err}", media.MediaId, result.Error);
			return;
		}
		var update = Builders<Media>.Update.Set(m => m.Cdn!.Url, $"/{result.Value}");
		await _media.UpdateOneAsync(m => m.MediaId == media.MediaId, update);
		await _gridFs.DeleteAsync(media.MediaId);

		await MigrateThumbnails(media);

		LogConversionSuccess(media.Filename, media.MediaId);
	}

	[LoggerMessage(Level = LogLevel.Information, Message = "Converted Media: {filename}. Url: /m/{id}")]
	private partial void LogConversionSuccess(string filename, ObjectId id);

	private async Task MigrateThumbnails(Media media)
	{
		var (thumbExt, mimeType) = media.Ext switch
		{
			".avif" => (".avif", "image/avif"),
			_ => (".webp", "image/webp")
		};
		var cdnThumbnails = new Dictionary<ThumbnailSize, string>();
		foreach (var (size, id) in media.Thumbnails) 
		{
			using var file = await _gridFs.OpenDownloadStreamAsync(id);
			var thumbUpload = await s3.UploadFileAsync($"{media.MediaId}/thumb/{size}{thumbExt}", mimeType, file);
			if (thumbUpload.HasError)
			{
				logger.LogError("Failed to upload thumb to s3 {id} | {thumb} - {err}", media.MediaId, id, thumbUpload.Error);
				continue;
			}
			cdnThumbnails.Add(size, thumbUpload);
			await _gridFs.DeleteAsync(id);
		}
		var update = Builders<Media>.Update.Set(m => m.Cdn!.ThumbnailUrls, cdnThumbnails);
		await _media.UpdateOneAsync(m => m.MediaId == media.MediaId, update);
	}
}

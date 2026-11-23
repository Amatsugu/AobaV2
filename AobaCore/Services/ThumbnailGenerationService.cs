using AobaCore.Models;
using AobaCore.ThumbnailGeneration;

using HeyRed.Mime;

using MaybeError.Errors;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;

using MongoDB.Bson;

namespace AobaCore.Services;

public class ThumbnailGenerationService(ThumbnailRequestService thumbnailRequests, AobaService aoba, S3MediaService s3, ILogger<ThumbnailGenerationService> logger) : IHostedService
{
	private Timer? _timer;

	public Task StartAsync(CancellationToken cancellationToken)
	{
		_timer = new Timer(_ =>
		{
			try
			{
				GenerateThumbnailsAsync(cancellationToken).GetAwaiter().GetResult();
			}
			catch (Exception e)
			{
				logger.LogError(e, "Failed to Generate Thumbails");
			}
			finally
			{
				_timer?.Change(TimeSpan.FromSeconds(30), Timeout.InfiniteTimeSpan);
			}
		}, null, TimeSpan.Zero, Timeout.InfiniteTimeSpan);
		return Task.CompletedTask;
	}

	private async Task GenerateThumbnailsAsync(CancellationToken cancellationToken = default)
	{
		var requests = await thumbnailRequests.GetRequestsAsync(100, cancellationToken);
		if (requests.Count > 0 && logger.IsEnabled(LogLevel.Information))
			logger.LogInformation("Generating thumbnails for {count} items", requests.Count);
		else
			return;
		var genCount = 0;
		while(requests.Count > 0)
		{
			foreach (var request in requests)
			{
				if (cancellationToken.IsCancellationRequested)
					break;
				try
				{
					await ProcressThumbnailRequestAsync(request);
					genCount++;
				}
				catch (Exception e)
				{
					await thumbnailRequests.MarkAsFailedAsync(request.RequestId, cancellationToken);
					logger.LogError(e, "Failed to generate thumbnail for {id} - {size}", request.MediaId, request.Size);
				}
			}
			requests = await thumbnailRequests.GetRequestsAsync(100, cancellationToken);
		}
		if(logger.IsEnabled(LogLevel.Information))
			logger.LogInformation("Generated thumbnails for {count} items.", genCount);
	}

	private async Task ProcressThumbnailRequestAsync(ThumbnailRequest request)
	{
		var media = await aoba.GetMediaAsync(request.MediaId);
		if (media?.Cdn == null || media.Cdn.ThumbnailUrls.ContainsKey(request.Size))
		{
			await thumbnailRequests.RemoveRequestAsync(request.RequestId);
			return;
		}
		var data = await s3.GetFileAsync(media.GetS3Filename());
		if (data.HasError)
		{
			logger.LogError(data.Error.Exception, "Failed to load file from s3 while generating thumbnail for {id} - {size}.", request.MediaId, request.Size);
			await thumbnailRequests.MarkAsFailedAsync(request.RequestId);
			return;
		}
		var filePath = $"/tmp/{ObjectId.GenerateNewId()}{media.Ext}";
		using (data.Value)
		{
			using var file = new FileStream(filePath, FileMode.Create);
			await data.Value.CopyToAsync(file);
			await file.FlushAsync();
		}

		if (media.Metadata == null)
		{
			var metadata = await GetMediaMetadataAsync(filePath, media);
			if(metadata.HasError)
			{
				logger.LogError(metadata.Error.GetException(), "Failed to parse metadata for {id}", request.MediaId);
				await thumbnailRequests.MarkAsFailedAsync(request.RequestId);
				File.Delete(filePath);
				return;
			}
			media.Metadata ??= metadata;
			await aoba.SetMetaMetadataAsync(media.MediaId, media.Metadata);
		}
		if (!ShouldGenerateThumb(media))
		{
			File.Delete(filePath);
			await thumbnailRequests.RemoveRequestAsync(request.RequestId);
			return;
		}

		var thumbnail = media.MediaType switch
		{
			MediaType.Audio => await Audio.GenerateThumbnailAsync(filePath, request.Size),
			MediaType.Image => Image.GenerateThumbnail(filePath, request.Size, media.Metadata),
			MediaType.Video => Video.GenerateThumbnail(filePath, request.Size, media.Metadata),
			_ => new Error($"Thumbnail generation is not possible for {media.MediaType} ({media.Ext})")
		};


		if (thumbnail.HasError)
		{
			logger.LogError(thumbnail.Error.GetException(), "Failed to generate thumbnail for {id} - {size}\n{details}", request.MediaId, request.Size, thumbnail.Error.Details);
			await thumbnailRequests.MarkAsFailedAsync(request.RequestId);
			File.Delete(filePath);
			return;
		}

		var thumbExt = media switch
		{
			{ MediaType: MediaType.Video } => ".webm",
			{ MediaType: MediaType.Image } => ".avif",
			{ MediaType: MediaType.Audio } => ".avif",
			_ => ".webp"
		};
		var thumbPath = $"{media.MediaId}/thumb/{request.Size}{thumbExt}";
		var s3Result = await s3.UploadFileAsync(thumbPath, MimeTypesMap.GetMimeType(thumbExt), thumbnail.Value);
		if (s3Result.HasError)
		{
			logger.LogError(s3Result.Error.GetException(), "Failed to upload thumbnail to s3 for {id}", request.MediaId);
			await thumbnailRequests.MarkAsFailedAsync(request.RequestId);
			File.Delete(filePath);
			return;
		}
		await aoba.AddS3ThumbnailAsync(media.MediaId, s3Result, request.Size);
		await thumbnailRequests.RemoveRequestAsync(request.RequestId);
		File.Delete(filePath);
	}

	private static bool ShouldGenerateThumb(Media media)
	{
		return media.MediaType switch
		{
			MediaType.Image => true,
			MediaType.Audio => true,
			MediaType.Video => true,
			MediaType.Text => true,
			MediaType.Code => true,
			_ => false,
		};
	}

	private static async Task<Maybe<MediaMetadata>> GetMediaMetadataAsync(string filePath, Media media)
	{
		return media switch
		{
			{ MediaType : MediaType.Audio } => await Audio.GetMetadataAsync(filePath),
			{ MediaType : MediaType.Video } => await Video.GetMetadataAsync(filePath),
			{ MediaType : MediaType.Image } => await Image.GetMetadataAsync(filePath),
			_ => new MediaMetadata
			{
				FileSize = new FileInfo(filePath).Length
			}
		};
	}

	public async Task StopAsync(CancellationToken cancellationToken)
	{
		if (_timer != null)
			await _timer.DisposeAsync();
	}
}
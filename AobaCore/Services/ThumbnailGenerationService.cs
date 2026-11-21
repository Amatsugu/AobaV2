using AobaCore.Models;
using MaybeError.Errors;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;

namespace AobaCore.Services;

public class ThumbnailGenerationService(ThumbnailRequestService thumbnailRequests, AobaService aoba, S3MediaService s3, ILogger<ThumbnailGenerationService> logger) : IHostedService
{
	private Timer? _timer;
	private bool _isRunning;

	public Task StartAsync(CancellationToken cancellationToken)
	{
		_timer = new Timer(_ =>
		{
			if (_isRunning)
				return;
			_isRunning = true;
			try
			{
				Task.Run(GenerateThumbnailsAsync);
			}
			catch (Exception e)
			{
				logger.LogError(e, "Failed to Generate Thumbails");
			}
			finally
			{
				_isRunning = false;
			}
		}, null, TimeSpan.Zero, TimeSpan.FromMinutes(1));
		return Task.CompletedTask;
	}

	private async Task GenerateThumbnailsAsync()
	{
		var requests = await thumbnailRequests.GetRequestsAsync(5);
		foreach (var request in requests)
		{
			try
			{
				await ProcressThumbnailRequestAsync(request);
			}
			catch (Exception e)
			{
				await thumbnailRequests.MarkAsFailedAsync(request.RequestId);
				logger.LogError(e, "Failed to generate thumbnail for {id} - {size}", request.MediaId, request.Size);
			}
		}
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
			logger.LogError(data.Error.Exception, "Failed to generate thumbnail for {id} - {size}", request.MediaId, request.Size);
			await thumbnailRequests.MarkAsFailedAsync(request.RequestId);
			return;
		}
		var filePath = $"/tmp/{media.MediaId}{media.Ext}";
		using (data.Value)
		{
			using var file = new FileStream(filePath, FileMode.Create);
			await data.Value.CopyToAsync(file);
			await file.FlushAsync();
		}

		media.Metadata ??= await GetMediaMetadataAsync(filePath);


		await thumbnailRequests.RemoveRequestAsync(request.RequestId);
		File.Delete(filePath);
	}

	private async Task<Maybe<MediaMetadata>> GetMediaMetadataAsync(string filePath)
	{
		return new Error("Todo: Process metadata");
	}

	public async Task StopAsync(CancellationToken cancellationToken)
	{
		if (_timer != null)
			await _timer.DisposeAsync();
	}
}
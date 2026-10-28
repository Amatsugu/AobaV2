using AobaCore.Models;
using AobaCore.Services;

using MaybeError;

namespace AobaServer.Services;

public class MediaMetadataService(S3MediaService s3, AobaService aoba, ILogger<MediaMetadataService> logger) : IHostedService
{
	private Timer? _metadataJob;
	private bool _isScanning = false;

	public Task StartAsync(CancellationToken cancellationToken)
	{
		_metadataJob = new Timer((_) =>
		{
			if (_isScanning)
				return;
			_isScanning = true;
			try
			{
				Task.Run(async () =>
				{
					await ProcessMediaAsync(cancellationToken);
				});
			}
			catch (Exception ex)
			{
				logger.LogError(ex, "Failded to process metadata");
			}
		}, null, TimeSpan.FromMinutes(5), TimeSpan.FromMinutes(5));
		return Task.CompletedTask;
	}

	private async Task ProcessMediaAsync(CancellationToken cancellationToken)
	{
		var items = await aoba.GetMediaWithoutMetadataAsync(20, cancellationToken);
		foreach (var item in items)
		{
			if (cancellationToken.IsCancellationRequested)
				break;
			await ProcessMediaEntryAsync(item);
		}
	}

	private async Task ProcessMediaEntryAsync(Media media)
	{
		var s3file = await s3.GetFileAsync(media.GetS3Filename());
		if (s3file.HasError)
		{
			logger.LogError(s3file.Error.GetException(), "Failed to retreive file from s3: {id}", media.MediaId);
			return;
		}

		var file = s3file.Value;
		var meta = await (media.MediaType switch
		{
			MediaType.Image => ProcessImageAsync(file),
			MediaType.Video => ProcessVideoAsync(file),
			MediaType.Audio => ProcessAudioAsync(file),
			_ => ProcessGenericFileAsync(file)
		});
		if (meta.HasError)
		{
			logger.LogError(meta.Error.GetException(), "Failed to process metadata for file: {id} - {type}", media.MediaId, media.MediaType);
		}
	}

	private static Task<Maybe<MediaMetadata>> ProcessGenericFileAsync(Stream file)
	{
		return Task.FromResult<Maybe<MediaMetadata>>(new MediaMetadata
		{
			FileSize = file.Length
		});
	}

	private static async Task<Maybe<MediaMetadata>> ProcessImageAsync(Stream file)
	{
		//todo:
		//exiftool -json -G1 -ImageWidth -ImageHeight -MIMEType -"XMP-hdrgm:all" -"*GainMap*" -MPImageType -TransferCharacteristics -ColorPrimaries -ProfileDescription -HDRHeadroom -HDRGain input.jpg
		var meta = await ProcessGenericFileAsync(file);
		if (meta.HasError)
			return meta;
		throw new NotImplementedException();
	}

	private static async Task<Maybe<MediaMetadata>> ProcessAudioAsync(Stream file)
	{
		//todo:
		//ffprobe -v error -select_streams a:0 -show_entries stream=codec_name,bit_rate,duration:format=duration,bit_rate -of json input.mp3
		var meta = await ProcessGenericFileAsync(file);
		if (meta.HasError)
			return meta;
		throw new NotImplementedException();
	}

	private static async Task<Maybe<MediaMetadata>> ProcessVideoAsync(Stream file)
	{
		//todo:
		//ffprobe -v error -select_streams v:0 -show_entries stream=width,height,pix_fmt,color_range,color_space,color_transfer,color_primaries,duration,bit_rate:stream_tags=BPS:format=duration -of json input.mp4
		var meta = await ProcessGenericFileAsync(file);
		if (meta.HasError)
			return meta;
		throw new NotImplementedException();
	}

	public async Task StopAsync(CancellationToken cancellationToken)
	{
		if (_metadataJob != null)
			await _metadataJob.DisposeAsync();
	}
}
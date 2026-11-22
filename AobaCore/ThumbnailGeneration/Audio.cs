using AobaCore.Models;

using FFMpegCore;
using FFMpegCore.Pipes;

using MaybeError.Errors;

using MongoDB.Bson;

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace AobaCore.ThumbnailGeneration;

public class Audio
{
	public static async Task<Maybe<MediaMetadata>> GetMetadataAsync(string filePath, CancellationToken cancellationToken = default)
	{
		var metadata = new MediaMetadata
		{
			FileSize = new FileInfo(filePath).Length
		};
		var processInfo = new ProcessStartInfo
		{
			FileName = "ffprobe",
			RedirectStandardOutput = true,
			RedirectStandardError = true,
			UseShellExecute = false,
		};
		var args = "-v error -select_streams a:0 -show_entries stream=codec_name,bit_rate,duration:format=duration,bit_rate -of json".Split(' ');
		foreach (var arg in args)
			processInfo.ArgumentList.Add(arg);
		processInfo.ArgumentList.Add(filePath);

		var process = Process.Start(processInfo);
		if (process == null)
			return new Error("Failed to start ffprobe");
		await process.WaitForExitAsync(cancellationToken);
		var json = await process.StandardOutput.ReadToEndAsync(cancellationToken);
		var data = JsonSerializer.Deserialize<FFProbeData>(json);

		if (data == null)
			return new Error("Failed to parse ffprobe output");



		var stream = data.Streams.FirstOrDefault();
		var bitrateString = data.Format?.BitRate;
		if (string.IsNullOrWhiteSpace(bitrateString))
			bitrateString = stream?.BitRate;

		var durationString = data.Format?.Duration;
		if(string.IsNullOrWhiteSpace(durationString))
			durationString = stream?.Duration;

		if (int.TryParse(bitrateString, out var bitrate))
			metadata.AudioBitrate = bitrate;
		else
			return new Error("Failed to parse bitrate");

		if (float.TryParse(durationString, out var duration))
			metadata.Duration = (long)duration;
		else
			return new Error("Failed to pase duration");

		return metadata;
	}

	public static async Task<Maybe<Stream>> GenerateThumbnailAsync(string filePath, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var (w, h) = size.GetThumbnailSize();
		var fn = ObjectId.GenerateNewId().ToString();

		try
		{
			var output = new MemoryStream();
			var result = await FFMpegArguments.FromFileInput(filePath, false)
				.OutputToPipe(new StreamPipeSink(output), opt =>
				{
					opt.WithCustomArgument($"-lavfi \"showspectrumpic=s={w}x{h}:legend=0:color=plasma:scale=log\"").ForceFormat("avif");
				}).ProcessAsynchronously(true, cancellationToken: cancellationToken);
			if (!result.Success)
				return new Error($"FFmpeg failed: {result.ExitCode}");
			output.Position = 0;
			return output;
		}
		catch (Exception ex)
		{
			return ex;
		}
	}

	private class FFProbeData
	{
		[JsonPropertyName("streams")]
		public Meta[] Streams { get; set; } = [];
		[JsonPropertyName("format")]
		public Meta? Format { get; set; }
	}


	private class Meta
	{
		[JsonPropertyName("duration")]
		public string? Duration { get; set; }

		[JsonPropertyName("bit_rate")]
		public string? BitRate { get; set; }
	}
}

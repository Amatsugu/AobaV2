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

	public static Maybe<Stream> GenerateAudioThumbnail(string filePath, ThumbnailSize size, string ext)
	{
		var w = (int)size;
		var fn = ObjectId.GenerateNewId().ToString();

		try
		{
			var output = new MemoryStream();
			FFMpegArguments.FromFileInput(filePath, false)
				.OutputToPipe(new StreamPipeSink(output), opt =>
				{
					opt.WithCustomArgument($"-lavfi \"showspectrumpic=s={w}x{w}:legend=0:color=plasma:scale=log\"").ForceFormat("webp");
				}).ProcessSynchronously();
			output.Position = 0;
			return output;
		}
		catch (Exception ex)
		{
			return ex;
		}
		finally
		{
			File.Delete(filePath);
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

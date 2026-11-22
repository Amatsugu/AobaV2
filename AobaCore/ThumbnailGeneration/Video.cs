using AobaCore.Models;

using FFMpegCore;
using FFMpegCore.Pipes;

using MaybeError.Errors;

using MongoDB.Bson;

using SixLabors.ImageSharp;

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;


namespace AobaCore.ThumbnailGeneration;

public static class Video
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
			RedirectStandardError = true,
			RedirectStandardOutput = true,
			UseShellExecute = false
		};
		var args = "-v error -select_streams v:0 -show_entries stream=width,height,pix_fmt,color_range,color_space,color_transfer,color_primaries,duration:format=duration -of json".Split(' ');
		foreach (var arg in args)
			processInfo.ArgumentList.Add(arg);
		processInfo.ArgumentList.Add(filePath);

		var process = Process.Start(processInfo);

		if (process == null)
			return new Error("Failed to start ffprobe");

		await process.WaitForExitAsync(cancellationToken);

		var stdOut = await process.StandardOutput.ReadToEndAsync(cancellationToken);
		var data = JsonSerializer.Deserialize<FFProbeData>(stdOut);
		if (data == null)
			return new Error("Failed to parse ffprobe output");

		if (data.Streams.Length == 0)
			return new Error("Failed to read data for any streams");

		metadata.Hdr = data.IsHdr();
		metadata.Dimensions = data.GetDimensions();

		var duration = data.Streams[0].Duration;
		if (duration == null || duration == "N/A")
			duration = data.Format?.Duration;

		if(float.TryParse(duration, out var dur))
			metadata.Duration = (long)dur;
		else
			return new Error("Failed to parse duration");

		return metadata;
	}

	public static Maybe<Stream> GenerateThumbnail(string filePath, ThumbnailSize size, MediaMetadata meta)
	{
		var (w, h) = size.GetThumbnailSize(meta.Dimensions);
		var fn = ObjectId.GenerateNewId().ToString();
		try
		{
			var output = new MemoryStream();
			var args = FFMpegArguments.FromFileInput(filePath, false, opt =>
			{
				opt.WithCustomArgument("-t 10");
			}).OutputToPipe(new StreamPipeSink(output), opt =>
			{
				opt.WithCustomArgument($"-vf \"scale={w}:{h}:force_original_aspect_ratio=increase:flags=lanczos,crop={w}:{h}\" -c:v libsvtav1 -crf 34 -preset 2 -an -pix_fmt yuv420p -r 15")
				.ForceFormat("webm");
			});
			var result = args.ProcessSynchronously(false);
			if (!result.Success)
				return new Error("Failed to process file", details: $"ffmpeg {args.Arguments}");
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
		public FFStream[] Streams { get; set; } = [];
		[JsonPropertyName("format")]
		public Format? Format { get; set; }

		public bool IsHdr()
		{
			var stream = Streams[0];
			return stream.ColorTransfer switch
			{
				"smpte2084" => true,
				"arib-std-b67" => true,
				_ => false
			};
		}

		public Size GetDimensions()
		{
			return new Size(Streams[0].Width, Streams[0].Height);
		}
	}

	private class FFStream
	{
		[JsonPropertyName("width")]
		public int Width { get; set; }

		[JsonPropertyName("height")]
		public int Height { get; set; }

		[JsonPropertyName("pix_fmt")]
		public string? PixFmt { get; set; }

		[JsonPropertyName("color_range")]
		public string? ColorRange { get; set; }

		[JsonPropertyName("color_space")]
		public string? ColorSpace { get; set; }

		[JsonPropertyName("color_transfer")]
		public string? ColorTransfer { get; set; }

		[JsonPropertyName("color_primaries")]
		public string? ColorPrimaries { get; set; }
		[JsonPropertyName("duration")]
		public string? Duration { get; set; }
	}

	private class Format
	{
		[JsonPropertyName("duration")]
		public string? Duration { get; set; }
	}
}

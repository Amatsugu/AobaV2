using AobaCore.Models;

using MaybeError.Errors;

using SixLabors.ImageSharp;

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;

namespace AobaCore.ThumbnailGeneration;

public static class Image
{
	public static async Task<Maybe<MediaMetadata>> GetMetadataAsync(string filePath, CancellationToken cancellationToken = default)
	{
		var metadata = new MediaMetadata
		{
			FileSize = new FileInfo(filePath).Length
		};

		var processInfo = new ProcessStartInfo
		{
			FileName = "exiftool",
			RedirectStandardError = true,
			RedirectStandardOutput = true,
			UseShellExecute = false
		};
		var args = "-json -G1 -ImageWidth -ImageHeight -MIMEType -\"XMP-hdrgm:all\" -\"*GainMap*\" -MPImageType -TransferCharacteristics -ColorPrimaries -ProfileDescription -HDRHeadroom -HDRGain -n".Split(' ');
		foreach (var arg in args) 
			processInfo.ArgumentList.Add(arg);
		processInfo.ArgumentList.Add(filePath);

		var process = Process.Start(processInfo);

		if (process == null)
			return new Error("Failed to start ffprobe");

		await process.WaitForExitAsync(cancellationToken);

		var stdOut = await process.StandardOutput.ReadToEndAsync(cancellationToken);

		var result = JsonSerializer.Deserialize<ExifData[]>(stdOut);
		if (result == null || result.Length == 0)
			return new Error("Failed to parse metadata");

		var exif = result[0];

		metadata.Hdr = exif.IsHdr();
		metadata.Dimensions = exif.GetDimensions();

		return metadata;
	}


	private class ExifData : Dictionary<string, JsonValue>
	{
		public bool IsHdr()
		{
			var isHdr = false;

			if(TryGetValue("QuickTime:TransferCharacteristics", out var val))
			{
				var transfer = val.Deserialize<int>();
				if (transfer == 16 || transfer == 18)
					isHdr = true;
			}
			if(Keys.Any(k => k.StartsWith("XMP-hdrgm:", StringComparison.OrdinalIgnoreCase)))
				isHdr = true;

			return isHdr;
		}

		public Size? GetDimensions()
		{
			if(TryGetValue("File:ImageWidth", out var widthVal) && TryGetValue("File:ImageHeight", out var heightVal))
				return new Size(widthVal.Deserialize<int>(), heightVal.Deserialize<int>());

			return null;
		}
	}
}

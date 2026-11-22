using AobaCore.Models;
using AobaCore.Tools;

using MaybeError.Errors;

using MongoDB.Bson;

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
		if (metadata.Dimensions == null)
			return new Error("Failed to read dimensions");

		return metadata;
	}

	public static Maybe<Stream> GenerateThumbnail(string filePath, ThumbnailSize size, MediaMetadata meta)
	{
		var (w, h) = size.GetThumbnailSize(meta.Dimensions);

		try
		{
			return AVIFTools.CropImage(filePath, w, h);
		}
		catch (Exception ex)
		{
			return ex;
		}
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
			JsonValue? width = this.FirstOrDefault(v => v.Key.EndsWith("ImageWidth")).Value;
			JsonValue? height = this.FirstOrDefault(v => v.Key.EndsWith("ImageHeight")).Value;

			if(width != null && height != null)
				return new Size(width.Deserialize<int>(), height.Deserialize<int>());

			return null;
		}
	}
}

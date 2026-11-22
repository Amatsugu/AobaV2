using FFMpegCore;
using FFMpegCore.Pipes;

using MaybeError.Errors;

using MongoDB.Bson;

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Text;
using System.Text.Json;

namespace AobaCore.Tools;

public static class AVIFTools
{
	public static Maybe<AvifColorInfo> ProbleColorInfo(string file)
	{
		var processInfo = new ProcessStartInfo
		{
			FileName = "ffprobe",
			RedirectStandardOutput = true,
			RedirectStandardError = true,
			UseShellExecute = false,
		};
		processInfo.ArgumentList.Add("-v");
		processInfo.ArgumentList.Add("error");
		processInfo.ArgumentList.Add("-select_streams");
		processInfo.ArgumentList.Add("v:0");
		processInfo.ArgumentList.Add("-show_entries");
		processInfo.ArgumentList.Add("stream=pix_fmt,color_transfer,color_primaries,color_space,color_range");
		processInfo.ArgumentList.Add("-of");
		processInfo.ArgumentList.Add("json");
		processInfo.ArgumentList.Add(file);

		var process = Process.Start(processInfo);

		if (process == null)
			return new Error("Failed to start ffprobe");


		var stdOut = process.StandardOutput.ReadToEnd();
		var stdErr = process.StandardError.ReadToEnd();
		process.WaitForExit();

		if (process.ExitCode != 0)
			return new Error($"ffprobe failed: exit code {process.ExitCode}", stdErr);

		using var jsonOutput = JsonDocument.Parse(stdOut);
		var streams = jsonOutput.RootElement.GetProperty("streams");
		if (streams.GetArrayLength() == 0)
			return new Error($"ffprobe found no streams in file {file}");

		var stream = streams[0];

		string Get(string name) => stream.TryGetProperty(name, out var val) && val.ValueKind == JsonValueKind.String ? val.GetString()! : "unknown";

		return new AvifColorInfo(Get("pix_fmt"), Get("color_transfer"), Get("color_primaries"), Get("color_space"), Get("color_range"));
	}

	public static Maybe<Stream> CropImage(string inputFile, int width, int height, int crf = 18)
	{
		var color = ProbleColorInfo(inputFile);
		if (color.HasError)
			return color.Error;
		var avifIn = inputFile.EndsWith(".avif");
		var outFile = $"/tmp/{ObjectId.GenerateNewId()}_thumb.avif";
		try
		{
			var colorSpaceOpt = MapColorSpaceOption(color.Value.ColorSpace);
			var colorTransferOpt = avifIn ? color.Value.ColorTransfer : MapColorTransferOption(color.Value.ColorTransfer);
			var vf = $"scale={width}:{height}:force_original_aspect_ratio=increase:flags=lanczos";

			var args = FFMpegArguments.FromFileInput(inputFile)
				.OutputToFile(outFile, true, opts =>
				{
					var colorInfo = color.Value;
					opts.WithCustomArgument($"-vf \"{vf}\"")
						.WithVideoCodec("libaom-av1")
						.WithCustomArgument($"-pix_fmt {colorInfo.PixFmt}")
						.WithConstantRateFactor(crf)
						.WithCustomArgument("-b:v 0")
						.WithCustomArgument("-cpu-used 4");
					if(avifIn)
						opts.WithCustomArgument("-still-picture 1");
					if (colorInfo.ColorPrimaries != "unknown")
						opts.WithCustomArgument($"-color_primaries {colorInfo.ColorPrimaries}");
					if (colorTransferOpt != "unknown")
						opts.WithCustomArgument($"-color_trc {colorTransferOpt}");
					if (colorInfo.ColorRange != "unknown")
						opts.WithCustomArgument($"-color_range {colorInfo.ColorRange}");
					if (colorSpaceOpt != null)
						opts.WithCustomArgument($"-colorspace {colorSpaceOpt}");
				});
			var result = args
				.ProcessSynchronously(false);
			if (!result.Success)
				return new Error($"Failed to process file\n{string.Join('\n', result.StandardError)}", details: $"ffmpeg {args.Arguments}");
			var outputStream = new MemoryStream();
			using var fs = new FileStream(outFile, FileMode.Open);
			fs.CopyTo(outputStream);
			fs.Flush();
			fs.Dispose();
			File.Delete(outFile);
			outputStream.Flush();
			outputStream.Position = 0;
			return outputStream;
		}
		catch (Exception ex)
		{
			return new ExceptionError(ex);
		}
	}

	public static string? MapColorSpaceOption(string colorSpace) => colorSpace switch
	{
		"gbr" => "rgb",
		"bt2020nc" => "bt2020nc",
		"bt2020c" => "bt2020c",
		"bt709" => "bt709",
		"smpte170m" => "smpte170m",
		"unknown" or "" => null,
		var other => other
	};

	public static string? MapColorTransferOption(string trc) => trc switch
	{
		"bt470m" => "gamma22",
		"bt470bg" => "gamma28",
		"unknown" or "" => null,
		var other => other
	};
}
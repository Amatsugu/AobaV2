using FFMpegCore;

using MaybeError.Errors;

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Text;
using System.Text.Json;

namespace AobaCore.Tools;

public static class AVIFTools
{
	public static async Task<Maybe<AvifColorInfo>> ProbleColorInfoAsync(string file, CancellationToken cancellationToken = default)
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

		await process.WaitForExitAsync(cancellationToken);

		var stdOut = await process.StandardOutput.ReadToEndAsync(cancellationToken);
		var stdErr = await process.StandardError.ReadToEndAsync(cancellationToken);

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

	public static async Task<Error?> CropImageAsync(string inputFile, string outputFile, int size, int crf = 18, CancellationToken cancellationToken = default)
	{
		var color = await ProbleColorInfoAsync(inputFile, cancellationToken);
		if (color.HasError)
			return color.Error;
		try
		{
			var colorSpaceOpt = MapColorSpaceOption(color.Value.ColorSpace);
			var vf = $"scale={size}:{size}:force_original_aspect_ratio=increase:flags=lanczos,crop={size}:{size}";

			var success = await FFMpegArguments.FromFileInput(inputFile)
				.OutputToFile(outputFile, overwrite: true, opts =>
				{
					var colorInfo = color.Value;
					opts.WithCustomArgument($"-vf \"{vf}\"")
						.WithVideoCodec("libaom-av1")
						.WithCustomArgument($"-pix_fmt {colorInfo.PixFmt}")
						.WithConstantRateFactor(crf)
						.WithCustomArgument("-b:v 0")
						.WithCustomArgument("-cpu-used 4")
						.WithCustomArgument("-still-picture 1");
					if (colorInfo.ColorPrimaries != "unknown")
						opts.WithCustomArgument($"-color_primaries {colorInfo.ColorPrimaries}");
					if (colorInfo.ColorTransfer != "unknown")
						opts.WithCustomArgument($"-color_trc {colorInfo.ColorTransfer}");
					if (colorInfo.ColorRange != "unknown")
						opts.WithCustomArgument($"-color_range {colorInfo.ColorRange}");
					if (colorSpaceOpt != null)
						opts.WithCustomArgument($"-colorspace {colorSpaceOpt}");
				}).CancellableThrough(cancellationToken)
				.ProcessAsynchronously();
			if (!success)
				return new Error("Failed to process file");
		}
		catch (Exception ex)
		{
			return new ExceptionError(ex);
		}
		return null;
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
}
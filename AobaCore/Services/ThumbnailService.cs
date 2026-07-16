using AobaCore.Models;
using AobaCore.Tools;

using FFMpegCore;
using FFMpegCore.Pipes;

using Flurl;

using HeyRed.Mime;

using MaybeError.Errors;

using MongoDB.Bson;
using MongoDB.Driver;
using MongoDB.Driver.GridFS;

using SixLabors.ImageSharp;
using SixLabors.ImageSharp.Processing;

using System.Diagnostics;

namespace AobaCore.Services;

public class ThumbnailService(IMongoDatabase db, AobaService aobaService, S3MediaService s3Media, HostInfo hostInfo)
{
	private readonly GridFSBucket _gridfs = new GridFSBucket(db);

	public async Task<Error?> DeleteThumbnailAsync(ObjectId mediaId, ThumbnailSize size)
	{
		var thumbId = await aobaService.GetThumbnailIdAsync(mediaId, size);
		if (thumbId == default)
			return null;
		try
		{
			await _gridfs.DeleteAsync(thumbId);
			await aobaService.RemoveThumbnailAsync(mediaId, size);
		}
		catch (GridFSFileNotFoundException)
		{
			//Ignore if the file was not found (somehow already deleted)
			await aobaService.RemoveThumbnailAsync(mediaId, size);
		}
		catch (Exception e)
		{
			return new ExceptionError(e);
		}
		return null;
	}

	public async Task<Error?> DeleteThumbnailDirectAsync(ObjectId thumbnailId)
	{
		try
		{
			await _gridfs.DeleteAsync(thumbnailId);
		}
		catch (GridFSFileNotFoundException)
		{
			//Ignore if the file was not found (somehow already deleted)
		}
		catch (Exception e)
		{
			return new ExceptionError(e);
		}
		return null;
	}

	/// <summary>
	///
	/// </summary>
	/// <param name="mediaId">Media id</param>
	/// <param name="size"></param>
	/// <param name="cancellationToken"></param>
	/// <returns></returns>
	public async Task<Maybe<string>> GetOrCreateThumbnailAsync(ObjectId mediaId, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var media = await aobaService.GetMediaAsync(mediaId, cancellationToken);
		if (media == null)
			return new Error("Media does not exist");
		var existingThumb = GetExistingThumbUrl(media, size);
		if (existingThumb != null)
			return existingThumb!;

		try
		{
			var mediaData = await GetMediaFileAsync(media, cancellationToken);
			if (mediaData.HasError)
				return mediaData.Error;
			using (mediaData.Value)
			{
				var thumb = await GenerateThumbnailAsync(mediaData.Value, size, media.MediaType, media.Ext, cancellationToken);

				if (thumb.HasError)
					return thumb.Error;
				using (thumb.Value)
				{
					cancellationToken.ThrowIfCancellationRequested();

					var thumbExt = media.Ext switch
					{
						".avif" => ".avif",
						_ => ".webp"
					};

					var thumbUrl = await UploadThumbnailAsync(media, size, thumb, $"{media.Filename}{media.Ext}", cancellationToken);

					return thumbUrl;
				}
			}
		}
		catch (Exception ex)
		{
			return ex;
		}
	}

	public string? GetExistingThumbUrl(Media media, ThumbnailSize size)
	{
		if (media.Cdn != null)
		{
			if (media.Cdn.ThumbnailUrls.ContainsKey(size))
				return media.GetThumbnailUrl(size, hostInfo);
		}
		else
		{
			if (media.Thumbnails.ContainsKey(size))
				return media.GetThumbnailUrl(size, hostInfo);
		}
		return null;
	}

	private async Task<Maybe<Stream>> GetMediaFileAsync(Media media, CancellationToken cancellationToken = default)
	{
		if (media.Cdn == null)
		{
			return await _gridfs.OpenDownloadStreamAsync(media.MediaId, new GridFSDownloadOptions { Seekable = true }, cancellationToken);
		}
		else
		{
			var result = await s3Media.GetFileAsync(media.GetS3Filename(), cancellationToken);
			if (result.HasError)
				return result.Error;
			return result.Value;
		}
	}

	private async Task<Maybe<string>> UploadThumbnailAsync(Media media, ThumbnailSize size, Stream file, string filename, CancellationToken cancellationToken = default)
	{
		try
		{
			if (media.Cdn != null)
			{
				var result = await s3Media.UploadFileAsync($"{media.MediaId}/thumb/{size}{Path.GetExtension(filename)}", MimeTypesMap.GetMimeType(filename), file, cancellationToken);
				if (result.HasError)
					return result.Error;
				await aobaService.AddS3ThumbnailAsync(media.MediaId, result, size, cancellationToken);
				return hostInfo.CdnHost.AppendPathSegments(result.Value).ToString();
			}
			else
			{
				var thumbId = await _gridfs.UploadFromStreamAsync(filename, file, cancellationToken: CancellationToken.None);
				await aobaService.AddThumbnailAsync(media.MediaId, thumbId, size, cancellationToken);
				return hostInfo.Host.AppendPathSegments("t", thumbId).ToString();
			}
		}
		catch (Exception ex)
		{
			return ex;
		}
	}

	/// <summary>
	///
	/// </summary>
	/// <param name="mediaId">Media Id</param>
	/// <param name="size"></param>
	/// <param name="cancellationToken"></param>
	/// <returns></returns>
	public async Task<(Stream? thumb, string? mimeType)> GetThumbnailAsync(ObjectId mediaId, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var thumb = await aobaService.GetThumbnailIdAsync(mediaId, size, cancellationToken);
		if (thumb == default)
			return (null, null);

		var thumbData = await _gridfs.OpenDownloadStreamAsync(thumb, cancellationToken: cancellationToken);
		return (thumbData, MimeTypesMap.GetMimeType(thumbData.FileInfo.Filename));
	}

	public async Task<(Stream? thumb, string? mimeType)> GetThumbnailByFileIdAsync(ObjectId thumbId, CancellationToken cancellationToken = default)
	{
		var thumbData = await _gridfs.OpenDownloadStreamAsync(thumbId, cancellationToken: cancellationToken);
		return (thumbData, MimeTypesMap.GetMimeType(thumbData.FileInfo.Filename));
	}

	public async Task<Maybe<Stream>> GenerateThumbnailAsync(Stream stream, ThumbnailSize size, MediaType type, string ext, CancellationToken cancellationToken = default)
	{
		return type switch
		{
			MediaType.Image => ext switch
			{
				".avif" => await GenerateAvifThumbnailV2Async(stream, size, cancellationToken),
				_ => await GenerateImageThumbnailAsync(stream, size, ext, cancellationToken),
			},
			MediaType.Video => GenerateVideoThumbnail(stream, size, cancellationToken),
			MediaType.Audio => GenerateAudioThumbnail(stream, size, ext, cancellationToken),
			MediaType.Text or MediaType.Code => await GenerateTextThumbnailAsync(stream, size, cancellationToken),
			_ => new Error($"No Thumbnail for {type}"),
		};
	}

	private static Maybe<Image> LoadImage(Stream stream, string ext)
	{
		if (ext is ".heif" or ".avif")
		{
			return new Error("Unsupported image type");
		}
		else
			return Image.Load(stream);
	}

	public static async Task<Maybe<Stream>> GenerateImageThumbnailAsync(Stream stream, ThumbnailSize size, string ext, CancellationToken cancellationToken = default)
	{
		var img = LoadImage(stream, ext);
		if (img.HasError)
			return img.Error;
		img.Value.Mutate(o =>
		{
			var size =
			o.Resize(new ResizeOptions
			{
				Position = AnchorPositionMode.Center,
				Mode = ResizeMode.Crop,
				Size = new Size(300, 300)
			});
		});
		var result = new MemoryStream();
		await img.Value.SaveAsWebpAsync(result, cancellationToken);
		img.Value.Dispose();
		result.Position = 0;
		return result;
	}

	public static Maybe<Stream> GenerateAudioThumbnail(Stream data, ThumbnailSize size, string ext, CancellationToken cancellationToken = default)
	{
		var w = (int)size;
		var fn = ObjectId.GenerateNewId().ToString();
		var filePath = $"/tmp/{fn}{ext}";

		using var source = new FileStream(filePath, FileMode.CreateNew);
		data.CopyTo(source);
		source.Flush();
		source.Dispose();
		data.Dispose();
		//ffmpeg -i test.wav -lavfi "showspectrumpic=s=512x512:legend=0:color=plasma:scale=log" output3.png
		try
		{
			var output = new MemoryStream();
			FFMpegArguments.FromFileInput(filePath, false)
				.OutputToPipe(new StreamPipeSink(output), opt =>
				{
					opt.WithCustomArgument("-lavfi \"showspectrumpic=s=512x512:legend=0:color=plasma:scale=log\"").ForceFormat("webp");
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

	public static Maybe<Stream> GenerateVideoThumbnail(Stream data, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var w = (int)size;
		var fn = ObjectId.GenerateNewId().ToString();
		var filePath = $"/tmp/{fn}.in";
		using var source = new FileStream(filePath, FileMode.CreateNew);
		data.CopyTo(source);
		source.Flush();
		source.Dispose();
		data.Dispose();
		try
		{
			var output = new MemoryStream();
			FFMpegArguments.FromFileInput(filePath, false, opt =>
			{
				opt.WithCustomArgument("-t 5");
			}).OutputToPipe(new StreamPipeSink(output), opt =>
			{
				opt.WithCustomArgument($"-vf \"crop='min(in_w,in_h)':'min(in_w,in_h)',scale={w}:{w}\" -loop 0 -r 15")
				.ForceFormat("webp");
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

	public static Maybe<Stream> GenerateAvifThumbnail(Stream data, ThumbnailSize size)
	{
		var w = (int)size;
		var fn = ObjectId.GenerateNewId().ToString();
		var inFilePath = $"/tmp/{fn}.avif";
		var outFilePath = $"/tmp/tn_{fn}.avif";
		using var source = new FileStream(inFilePath, FileMode.CreateNew);
		data.CopyTo(source);
		source.Flush();
		source.Dispose();
		source.Close();
		data.Dispose();
		try
		{
			var process = Process.Start(new ProcessStartInfo
			{
#if DEBUG
				FileName = "H:\\Tools\\vips-dev-8.18\\bin\\vipsthumbnail.exe",
#else
				FileName = "vipsthumbnail",
#endif
				Arguments = $"\"{inFilePath}\" --size {w} --crop --output {outFilePath}",
				//WorkingDirectory = "/tmp"
			});
			if (process == null)
				return new Error("Failed to run vips command");
			process.WaitForExit();
			if (process.ExitCode != 0)
			{
				var err = process.StandardError.ReadToEnd();
				return new Error("Failed to convert", err);
			}
			var output = new MemoryStream();
			using var oFile = File.OpenRead(outFilePath);
			oFile.CopyTo(output);
			output.Position = 0;
			return output;
		}
		catch (Exception ex)
		{
			return ex;
		}
		finally
		{
			File.Delete(inFilePath);
			File.Delete(outFilePath);
		}
	}

	public static async Task<Maybe<Stream>> GenerateAvifThumbnailV2Async(Stream data, ThumbnailSize size, CancellationToken cancellationToken)
	{
		var w = (int)size;
		var fn = ObjectId.GenerateNewId().ToString();
		var inFilePath = $"/tmp/{fn}.avif";
		var outFilePath = $"/tmp/thumb_{fn}.avif";
		using var source = new FileStream(inFilePath, FileMode.CreateNew);
		data.CopyTo(source);
		source.Flush();
		source.Dispose();
		source.Close();
		data.Dispose();
		try
		{
			var error = await AVIFTools.CropImageAsync(inFilePath, outFilePath, w, cancellationToken: cancellationToken);
			if (error != null)
				return error;
			var output = new MemoryStream();
			using var oFile = File.OpenRead(outFilePath);
			oFile.CopyTo(output);
			output.Position = 0;
			return output;
		}
		catch(Exception ex)
		{
			return ex;
		}
		finally
		{
			File.Delete(inFilePath);
			File.Delete(outFilePath);
		}
	}

	public async Task<Maybe<Stream>> GenerateTextThumbnailAsync(Stream data, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		//var w = (int)size;
		//using var image = new Image<Rgba32>(w, w);
		//var reader = new StreamReader(data);
		//var text = new char[500];
		//reader.ReadBlock(text, 0, text.Length);
		//image.Mutate(op =>
		//{
		//	op.BackgroundColor(Color.Black);
		//	var font = new Font(), 11);
		//	var textOpts = new RichTextOptions(font);
		//	op.DrawText(, new string(text), new Brush
		//	{
		//	});
		//});
		return new NotImplementedException();
	}
}
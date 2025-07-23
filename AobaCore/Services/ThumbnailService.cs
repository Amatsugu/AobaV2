using AobaCore.Models;

using FFMpegCore;
using FFMpegCore.Pipes;


using MaybeError.Errors;

using MongoDB.Bson;
using MongoDB.Driver;
using MongoDB.Driver.GridFS;

using SixLabors.ImageSharp;
using SixLabors.ImageSharp.Formats;
using SixLabors.ImageSharp.Processing;

using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http.Headers;
using System.Text;
using System.Threading.Tasks;

namespace AobaCore.Services;

public class ThumbnailService(IMongoDatabase db, AobaService aobaService)
{
	private readonly GridFSBucket _gridfs = new GridFSBucket(db);

	/// <summary>
	///
	/// </summary>
	/// <param name="mediaId">Media id</param>
	/// <param name="size"></param>
	/// <param name="cancellationToken"></param>
	/// <returns></returns>
	public async Task<Maybe<Stream>> GetOrCreateThumbnailAsync(ObjectId mediaId, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var existingThumb = await GetThumbnailAsync(mediaId, size, cancellationToken);
		if (existingThumb != null)
			return existingThumb;

		var media = await aobaService.GetMediaFromFileAsync(mediaId, cancellationToken);

		if (media == null)
			return new Error("Media does not exist");

		try
		{
			using var mediaData = await _gridfs.OpenDownloadStreamAsync(media.MediaId, new GridFSDownloadOptions { Seekable = true }, cancellationToken);
			var thumb = await GenerateThumbnailAsync(mediaData, size, media.MediaType, media.Ext, cancellationToken);

			if (thumb.HasError)
				return thumb.Error;
			cancellationToken.ThrowIfCancellationRequested();

			var thumbId = await _gridfs.UploadFromStreamAsync($"{media.Filename}.webp", thumb, cancellationToken: CancellationToken.None);
			await aobaService.AddThumbnailAsync(mediaId, thumbId, size, cancellationToken);

			thumb.Value.Position = 0;
			return thumb;
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
	public async Task<Stream?> GetThumbnailAsync(ObjectId mediaId, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var thumb = await aobaService.GetThumbnailIdAsync(mediaId, size, cancellationToken);
		if (thumb == default)
			return null;

		var thumbData = await _gridfs.OpenDownloadStreamAsync(thumb, cancellationToken: cancellationToken);
		return thumbData;
	}

	public async Task<Stream?> GetThumbnailByFileIdAsync(ObjectId thumbId, CancellationToken cancellationToken = default)
	{
		var thumbData = await _gridfs.OpenDownloadStreamAsync(thumbId, cancellationToken: cancellationToken);
		return thumbData;
	}

	public async Task<Maybe<Stream>> GenerateThumbnailAsync(Stream stream, ThumbnailSize size, MediaType type, string ext, CancellationToken cancellationToken = default)
	{
		return type switch
		{
			MediaType.Image => await GenerateImageThumbnailAsync(stream, size, ext, cancellationToken),
			MediaType.Video => GenerateVideoThumbnail(stream, size, cancellationToken),
			MediaType.Text or MediaType.Code => await GenerateDocumentThumbnailAsync(stream, size, cancellationToken),
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
		img.Value.Dispose();
		var result = new MemoryStream();
		await img.Value.SaveAsWebpAsync(result, cancellationToken);
		result.Position = 0;
		return result;
	}

	public Maybe<Stream> GenerateVideoThumbnail(Stream data, ThumbnailSize size, CancellationToken cancellationToken = default)
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

	public async Task<Maybe<Stream>> GenerateDocumentThumbnailAsync(Stream data, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		return new NotImplementedException();
	}
}
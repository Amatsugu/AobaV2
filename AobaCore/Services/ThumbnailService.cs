using AobaCore.Models;

using FFMpegCore;
using FFMpegCore.Pipes;

using MaybeError.Errors;

using MongoDB.Bson;
using MongoDB.Driver;
using MongoDB.Driver.GridFS;

using SixLabors.ImageSharp;
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
	private readonly IMongoCollection<MediaThumbnail> _thumbnails = db.GetCollection<MediaThumbnail>("thumbs");


	/// <summary>
	/// 
	/// </summary>
	/// <param name="id">File id</param>
	/// <param name="size"></param>
	/// <param name="cancellationToken"></param>
	/// <returns></returns>
	public async Task<Maybe<Stream>> GetOrCreateThumbnailAsync(ObjectId id, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var existingThumb = await GetThumbnailAsync(id, size, cancellationToken);
		if (existingThumb != null)
			return existingThumb;

		var media = await aobaService.GetMediaFromFileAsync(id, cancellationToken);

		if (media == null)
			return new Error("Media does not exist");

		try
		{

			using var mediaData = await _gridfs.OpenDownloadStreamAsync(media.MediaId, new GridFSDownloadOptions { Seekable = true }, cancellationToken);
			var thumb = await GenerateThumbnailAsync(mediaData, size, media.MediaType, media.Ext, cancellationToken);

			if (thumb.HasError)
				return thumb.Error;
			cancellationToken.ThrowIfCancellationRequested();

#if !DEBUG
			var thumbId = await _gridfs.UploadFromStreamAsync($"{media.Filename}.webp", thumb, cancellationToken: CancellationToken.None);
			var update = Builders<MediaThumbnail>.Update.Set(t => t.Sizes[size], thumbId);
			await _thumbnails.UpdateOneAsync(t => t.Id == id, update, cancellationToken: CancellationToken.None);
#endif
			thumb.Value.Position = 0;
			return thumb;
		} catch (Exception ex) {
			return ex;
		}
		

		
	}

	/// <summary>
	/// 
	/// </summary>
	/// <param name="id">File Id</param>
	/// <param name="size"></param>
	/// <param name="cancellationToken"></param>
	/// <returns></returns>
	public async Task<Stream?> GetThumbnailAsync(ObjectId id, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var thumb = await _thumbnails.Find(t => t.Id == id).FirstOrDefaultAsync(cancellationToken);
		if (thumb == null) 
			return null;

		if (!thumb.Sizes.TryGetValue(size, out var tid))
			return null;

		var thumbData = await _gridfs.OpenDownloadStreamAsync(tid, cancellationToken: cancellationToken);
		return thumbData;
	}


	public async Task<Maybe<Stream>> GenerateThumbnailAsync(Stream stream, ThumbnailSize size, MediaType type, string ext, CancellationToken cancellationToken = default)
	{
		return type switch
		{
			MediaType.Image => await GenerateImageThumbnailAsync(stream, size, cancellationToken),
			MediaType.Video => await GenerateVideoThumbnailAsync(stream, size, cancellationToken),
			MediaType.Text or MediaType.Code => await GenerateDocumentThumbnailAsync(stream, size, cancellationToken),
			_ => new Error($"No Thumbnail for {type}"),
		};
	}

	public async Task<Stream> GenerateImageThumbnailAsync(Stream stream, ThumbnailSize size, CancellationToken cancellationToken = default) 
	{
		var img = Image.Load(stream);
		img.Mutate(o =>
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
		await img.SaveAsWebpAsync(result, cancellationToken);
		result.Position = 0;
		return result;
	}

	public async Task<Maybe<Stream>> GenerateVideoThumbnailAsync(Stream data, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var w = (int)size;
		var source = new MemoryStream();
		data.CopyTo(source);
		source.Position = 0;
		var output = new MemoryStream();
		await FFMpegArguments.FromPipeInput(new StreamPipeSource(source))
				.OutputToPipe(new StreamPipeSink(output), opt =>
				{
					opt.WithCustomArgument($"-t 5 -vf \"crop='min(in_w,in_h)':'min(in_w,in_h)',scale={w}:{w}\" -loop 0")
					.ForceFormat("webp");
				}).ProcessAsynchronously();
		output.Position = 0;

		return output;
	}

	public async Task<Maybe<Stream>> GenerateDocumentThumbnailAsync(Stream data, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		return new NotImplementedException();
	}
}

using AobaCore.Models;

using MongoDB.Bson;
using MongoDB.Driver;
using MongoDB.Driver.GridFS;

using SixLabors.ImageSharp;
using SixLabors.ImageSharp.Processing;

using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AobaCore.Services;
public class ThumbnailService(IMongoDatabase db, AobaService aobaService)
{
	private readonly GridFSBucket _gridfs = new GridFSBucket(db);
	private readonly IMongoCollection<MeidaThumbnail> _thumbnails = db.GetCollection<MeidaThumbnail>("thumbs");

	public async Task<Stream?> GetThumbnailAsync(ObjectId id, CancellationToken cancellationToken = default)
	{
		var media = await aobaService.GetMediaAsync(id);
		if (media == null)
			return null;
		if (media.MediaType != MediaType.Image)
			return null;
		using var file = await _gridfs.OpenDownloadStreamAsync(media.MediaId, new GridFSDownloadOptions { Seekable = true });
		return await GenerateThumbnailAsync(file, cancellationToken);
	}
	public async Task<Stream?> GetThumbnailFromFileAsync(ObjectId id, CancellationToken cancellationToken = default)
	{
		var media = await aobaService.GetMediaFromFileAsync(id);
		if (media == null)
			return null;
		if (media.MediaType != MediaType.Image)
			return null;
		using var file = await _gridfs.OpenDownloadStreamAsync(media.MediaId, new GridFSDownloadOptions { Seekable = true });
		return await GenerateThumbnailAsync(file, cancellationToken);
	}

	public async Task<Stream> GenerateThumbnailAsync(Stream stream, CancellationToken cancellationToken = default) 
	{
		var img = Image.Load(stream);
		img.Mutate(o =>
		{
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
}

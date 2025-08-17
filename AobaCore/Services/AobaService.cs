using AobaCore.Models;

using MaybeError.Errors;

using Microsoft.Extensions.Logging;

using MongoDB.Bson;
using MongoDB.Driver;
using MongoDB.Driver.GridFS;

namespace AobaCore.Services;

public class AobaService(IMongoDatabase db)
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>("media");
	private readonly GridFSBucket _gridFs = new(db);

	public async Task<Media?> GetMediaFromLegacyIdAsync(ObjectId id, CancellationToken cancellationToken = default)
	{
		return await _media.Find(m => m.LegacyId == id).FirstOrDefaultAsync(cancellationToken);
	}

	public async Task<Media?> GetMediaFromFileAsync(ObjectId id, CancellationToken cancellationToken = default)
	{
		return await _media.Find(m => m.MediaId == id).FirstOrDefaultAsync(cancellationToken);
	}

	public async Task<PagedResult<Media>> FindMediaAsync(string? query, ObjectId userId, int page = 1, int pageSize = 100)
	{
		var filter = Builders<Media>.Filter.And([
			string.IsNullOrWhiteSpace(query) ? "{}" : Builders<Media>.Filter.Text(query),
			Builders<Media>.Filter.Eq(m => m.Owner, userId)
		]);
		var sort = Builders<Media>.Sort.Descending(m => m.UploadDate);
		var find = _media.Find(filter);

		var total = await find.CountDocumentsAsync();
		page -= 1;
		var items = await find.Sort(sort).Skip(page * pageSize).Limit(pageSize).ToListAsync();
		return new PagedResult<Media>(items, page, pageSize, total);
	}

	public async Task<List<Media>> FindMediaWithExtAsync(string ext, CancellationToken cancellationToken = default)
	{
		var filter = Builders<Media>.Filter.Eq(m => m.Ext, ext);
		return await _media.Find(filter).ToListAsync();
	}

	public Task AddMediaAsync(Media media, CancellationToken cancellationToken = default)
	{
		return _media.InsertOneAsync(media, null, cancellationToken);
	}

	public async Task AddThumbnailAsync(ObjectId mediaId, ObjectId thumbId, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var upate = Builders<Media>.Update.Set(m => m.Thumbnails[size], thumbId);

		await _media.UpdateOneAsync(m => m.MediaId == mediaId, upate, cancellationToken: cancellationToken);
	}

	public async Task RemoveThumbnailAsync(ObjectId mediaId, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var upate = Builders<Media>.Update.Unset(m => m.Thumbnails[size]);

		await _media.UpdateOneAsync(m => m.MediaId == mediaId, upate, cancellationToken: cancellationToken);
	}

	public async Task<ObjectId> GetThumbnailIdAsync(ObjectId mediaId, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var thumb = await _media.Find(m => m.MediaId == mediaId).Project(m => m.Thumbnails[size]).FirstOrDefaultAsync(cancellationToken);
		return thumb;
	}

	public Task IncrementViewCountAsync(ObjectId mediaId, CancellationToken cancellationToken = default)
	{
		return _media.UpdateOneAsync(m => m.MediaId == mediaId, Builders<Media>.Update.Inc(m => m.ViewCount, 1), cancellationToken: cancellationToken);
	}


	public async Task<Maybe<Media>> UploadFileAsync(Stream data, string filename, ObjectId owner, CancellationToken cancellationToken = default)
	{
		try
		{
			var fileId = await _gridFs.UploadFromStreamAsync(filename, data, cancellationToken: cancellationToken);
			var media = new Media(fileId, filename, owner);
			
			await AddMediaAsync(media, cancellationToken);
			return media;
		}
		catch (Exception ex)
		{
			return ex;
		}
		finally
		{
			data.Dispose();
		}
	}

	public async Task<MaybeEx<GridFSDownloadStream, GridFSException>> GetFileStreamAsync(ObjectId mediaId, bool seekable = false, CancellationToken cancellationToken = default)
	{
		try
		{
			return await _gridFs.OpenDownloadStreamAsync(mediaId, new GridFSDownloadOptions { Seekable = seekable }, cancellationToken);
		}
		catch (GridFSException ex)
		{
			return ex;
		}
	}

	public async Task DeleteFileAsync(ObjectId mediaId, CancellationToken cancellationToken = default)
	{
		try
		{
			cancellationToken.ThrowIfCancellationRequested();
			await _gridFs.DeleteAsync(mediaId, CancellationToken.None);
			await _media.DeleteOneAsync(m => m.MediaId == mediaId, CancellationToken.None);
		}
		catch (GridFSFileNotFoundException)
		{
			//ignore if file was not found
		}
	}

	

	public async Task DeriveTagsAsync(CancellationToken cancellationToken = default)
	{
		var mediaItems = await _media.Find(Builders<Media>.Filter.Exists(m => m.Tags, false))
			.ToListAsync(cancellationToken);
		Console.WriteLine($"Derving Tag for {mediaItems.Count} items");
		foreach (var mediaItem in mediaItems)
		{
			mediaItem.Tags = Media.DeriveTags(mediaItem.Filename);
			await _media.UpdateOneAsync(m => m.MediaId == mediaItem.MediaId, Builders<Media>.Update.Set(m => m.Tags, mediaItem.Tags), null, cancellationToken);
		}
		Console.WriteLine("All Tags Derived");
	}
}

using AobaCore.Models;

using MaybeError.Errors;

using MongoDB.Bson;
using MongoDB.Driver;
using MongoDB.Driver.GridFS;

namespace AobaCore;

public class AobaService(IMongoDatabase db)
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>("media");
	private readonly GridFSBucket _gridFs = new(db);

	public async Task<Media?> GetMediaAsync(ObjectId id, CancellationToken cancellationToken = default)
	{
		return await _media.Find(m => m.Id == id).FirstOrDefaultAsync(cancellationToken);
	}

	public async Task<PagedResult<Media>> FindMediaAsync(string? query, int page = 1, int pageSize = 50)
	{
		var filter = string.IsNullOrWhiteSpace(query) ? "{}" : Builders<Media>.Filter.Text(query);
		var find = _media.Find(filter);

		var total = await find.CountDocumentsAsync();
		page -= 1;
		var items = await find.Skip(page * pageSize).Limit(pageSize).ToListAsync();
		return new PagedResult<Media>(items, page, pageSize, total);
	}


	public Task AddMediaAsync(Media media, CancellationToken cancellationToken = default)
	{
		return _media.InsertOneAsync(media, null, cancellationToken);
	}
	
	public Task IncrementViewCountAsync(ObjectId id, CancellationToken cancellationToken = default)
	{
		return _media.UpdateOneAsync(m => m.Id == id, Builders<Media>.Update.Inc(m => m.ViewCount, 1), cancellationToken: cancellationToken);
	}

	public Task IncrementFileViewCountAsync(ObjectId fileId, CancellationToken cancellationToken = default)
	{
		return _media.UpdateOneAsync(m => m.MediaId == fileId, Builders<Media>.Update.Inc(m => m.ViewCount, 1), cancellationToken: cancellationToken);
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
	}

	public async Task<MaybeEx<GridFSDownloadStream, GridFSException>> GetFileStreamAsync(ObjectId id, bool seekable = false, CancellationToken cancellationToken = default)
	{
		try
		{
			return await _gridFs.OpenDownloadStreamAsync(id, new GridFSDownloadOptions { Seekable = seekable }, cancellationToken);
		}
		catch (GridFSException ex)
		{
			return ex;
		}
	}

	public async Task DeleteFileAsync(ObjectId fileId, CancellationToken cancellationToken = default)
	{
		try
		{
			await _gridFs.DeleteAsync(fileId, cancellationToken);
			await _media.DeleteOneAsync(m => m.MediaId == fileId, cancellationToken);
		}
		catch (GridFSFileNotFoundException)
		{
			//ignore if file was not found
		}
	}
}

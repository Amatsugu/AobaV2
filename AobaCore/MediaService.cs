using AobaV2.Models;

using MaybeError.Errors;

using MongoDB.Bson;
using MongoDB.Driver;
using MongoDB.Driver.GridFS;


namespace AobaCore;
public class MediaService(IMongoDatabase db, AobaService aobaService)
{
	private readonly GridFSBucket _gridFs = new(db);

	public async Task<Maybe<Media>> UploadMediaAsync(Stream data, string filename, ObjectId owner, CancellationToken cancellationToken = default)
	{
		var fileId = await _gridFs.UploadFromStreamAsync(filename, data, cancellationToken: cancellationToken);
		var media = new Media(fileId, filename, owner);
		await aobaService.AddMediaAsync(media);
		return media;
	}

	public async Task<Maybe<GridFSDownloadStream, ExceptionError<GridFSException>>> GetMediaStreamAsync(ObjectId id, bool seekable = false)
	{
		try
		{
			return await _gridFs.OpenDownloadStreamAsync(id, new GridFSDownloadOptions { Seekable = seekable });
		}
		catch (GridFSException ex)
		{
			return new ExceptionError<GridFSException>(ex);
		}
	}
}

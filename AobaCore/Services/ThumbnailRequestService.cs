using AobaCore.Models;

using Flurl;

using MaybeError.Errors;
using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;
using MongoDB.Driver;

namespace AobaCore.Services;

public class ThumbnailRequestService(IMongoDatabase db, AobaService aoba, S3MediaService s3, HostInfo hostInfo)
{
	private readonly IMongoCollection<ThumbnailRequest> _requests = db.GetCollection<ThumbnailRequest>(Collections.ThumbnailRequests);

	public async Task EnsureThumbnailsAsync(List<Media> items, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		foreach (var media in items)
		{
			if (media.Cdn != null && media.Cdn.ThumbnailUrls.ContainsKey(size))
				continue;
			await RequestThumbnailGenerationAsync(media.MediaId, size, cancellationToken);
		}
	}
	public async Task EnsureThumbnailAsync(Media media, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		if (media.Cdn != null && media.Cdn.ThumbnailUrls.ContainsKey(size))
			return;
		await RequestThumbnailGenerationAsync(media.MediaId, size, cancellationToken);
	}

	public async Task<string?> GetOrCreateThumbnailAsync(ObjectId mediaId, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var media = await aoba.GetMediaAsync(mediaId, cancellationToken);
		if (media == null)
			return null;
		if (media.Cdn != null && media.Cdn.ThumbnailUrls.TryGetValue(size, out var thumb))
			return hostInfo.CdnHost.AppendPathSegments(thumb);
		await RequestThumbnailGenerationAsync(mediaId, size, cancellationToken);
		return null;
	}

	public async Task<List<ThumbnailRequest>> GetRequestsAsync(int count, CancellationToken cancellationToken = default)
	{
		return await _requests.Find(r => !r.Failed).Limit(count).ToListAsync(cancellationToken);
	}

	public async Task<bool> RequestExistsAsync(ObjectId mediaId, ThumbnailSize size, CancellationToken cancellationToken)
	{
		return await _requests.Find(m => m.MediaId == mediaId && m.Size == size).AnyAsync(cancellationToken);
	}

	public async Task MarkAsFailedAsync(ObjectId requestId, CancellationToken cancellationToken = default)
	{
		//Remove failed requests for now
		await RemoveRequestAsync(requestId, cancellationToken);
		//var update = Builders<ThumbnailRequest>.Update.Set(r => r.Failed, true);
		//await _requests.UpdateOneAsync(r => r.RequestId == requestId, update, null, cancellationToken);
	}

	public async Task RemoveRequestAsync(ObjectId requestId, CancellationToken cancellationToken = default)
	{
		await _requests.DeleteOneAsync(r => r.RequestId == requestId, null, cancellationToken);
	}
	private async Task RequestThumbnailGenerationAsync(ObjectId mediaId, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var req = new ThumbnailRequest
		{
			MediaId = mediaId,
			Size = size,
		};
		try
		{
			await _requests.InsertOneAsync(req, null, cancellationToken);
		}
		catch
		{
			//Ignore duplicate requests
		}
	}

	public async Task<Error?> DeleteAllThumbnailsAsync(ObjectId mediaId)
	{
		var media = await aoba.GetMediaAsync(mediaId);
		if (media == null)
			return null;
		try
		{
			if (media.Cdn != null)
			{
				foreach (var (_, thumb) in media.Cdn.ThumbnailUrls)
					await s3.DeleteFileAsync(thumb);
				await aoba.ClearS3ThumbnailsAsync(mediaId);
			}
			return null;
		}
		catch (Exception ex)
		{
			return new ExceptionError(ex);
		}
	}

	public async Task<Error?> DeleteThumbnailAsync(ObjectId mediaId, ThumbnailSize size)
	{
		var media = await aoba.GetMediaAsync(mediaId);
		if (media == null)
			return null;
		try
		{
			if (media.Cdn != null && media.Cdn.ThumbnailUrls.TryGetValue(size, out var url))
			{
				await s3.DeleteFileAsync(url);
				await aoba.RemoveS3ThumbnailAsync(mediaId, size);
			}
			return null;
		}
		catch (Exception ex)
		{
			return new ExceptionError(ex);
		}
	}

}


using AobaCore.Models;
using MaybeError.Errors;
using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;
using MongoDB.Driver;

namespace AobaCore.Services;

public class ThumbnailRequestService(IMongoDatabase db, AobaService aoba)
{
	private readonly IMongoCollection<ThumbnailRequest> _requests = db.GetCollection<ThumbnailRequest>(Collections.ThumbnailRequests);


	public async Task<object?> GetOrCreateThumbnailAsync(ObjectId mediaId, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		var media = await aoba.GetMediaAsync(mediaId, cancellationToken);
		if (media == null)
			return null;
		if (media.Cdn != null && media.Cdn.ThumbnailUrls.TryGetValue(size, out var thumb))
			return thumb;
		await RequestThumbnailGenerationAsync(mediaId, size, cancellationToken);
		return null;
	}

	public async Task<List<ThumbnailRequest>> GetRequestsAsync(int count, CancellationToken cancellationToken = default)
	{
		return await _requests.Find(r => !r.Failed).Limit(count).ToListAsync(cancellationToken);
	}

	public async Task MarkAsFailedAsync(ObjectId requestId, CancellationToken cancellationToken = default)
	{
		var update = Builders<ThumbnailRequest>.Update.Set(r => r.Failed, true);
		await _requests.UpdateOneAsync(r => r.RequestId == requestId, update, null, cancellationToken);
	}

	public async Task RemoveRequestAsync(ObjectId requestId, CancellationToken cancellationToken = default)
	{
		await _requests.DeleteOneAsync(r => r.RequestId == requestId, null, cancellationToken);
	}
	private async Task RequestThumbnailGenerationAsync(ObjectId mediaId, ThumbnailSize size, CancellationToken cancellationToken = default)
	{
		await _requests.InsertOneAsync(new ThumbnailRequest
		{
			MediaId = mediaId,
			Size = size,
		});
	}

}


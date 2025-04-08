using AobaV2.Models;

using MongoDB.Bson;
using MongoDB.Driver;

namespace AobaCore;

public class AobaService(IMongoDatabase db)
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>("media");


	public async Task<Media?> GetMediaAsync(ObjectId id)
	{
		return await _media.Find(m => m.Id == id).FirstOrDefaultAsync();
	}

	public Task AddMediaAsync(Media media)
	{
		return _media.InsertOneAsync(media);
	}
}

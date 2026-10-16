using AobaCore.Models;
using MongoDB.Driver;

namespace AobaCore.Services;

public class MediaCollectionService(IMongoDatabase db)
{
	public IMongoCollection<MediaCollection> _collections = db.GetCollection<MediaCollection>("mediaCollections");
}
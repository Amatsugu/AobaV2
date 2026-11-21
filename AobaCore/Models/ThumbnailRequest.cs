
using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;

namespace AobaCore.Models;

public class ThumbnailRequest
{
	[BsonId]
	public ObjectId RequestId { get; set; }
	public ObjectId MediaId { get; set; }
	public ThumbnailSize Size { get; set; }
	public bool Failed { get; set; }
}
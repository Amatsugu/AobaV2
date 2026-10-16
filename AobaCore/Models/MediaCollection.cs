using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;

namespace AobaCore.Models;

public class MediaCollection
{
	[BsonId]
	public ObjectId Id { get; set; }
	public ObjectId OwnerId { get; set; }
	public string Title { get; set; }
	public string? Description { get; set; }
	public List<ObjectId> MediaItems { get; set; } = [];
	public List<string> Tags { get; set; } = [];
	public bool IsPublic { get; set; }
	public DateTime CreationDate { get; set; }

	[BsonConstructor]
	private MediaCollection()
	{
		Title = "Untitled Collection";
	}

	public MediaCollection(string title, ObjectId ownerId)
	{
		Title = title;
		OwnerId = ownerId;
	}
}
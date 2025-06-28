using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;

namespace AobaCore.Models;

public record MediaThumbnail
{
	[BsonId]
	public required ObjectId Id { get; init; }
	public Dictionary<ThumbnailSize, ObjectId> Sizes { get; set; } = [];
}

public enum ThumbnailSize
{
	Small = 128,
	Medium = 256,
	Large = 512,
	ExtraLarge = 1024
}
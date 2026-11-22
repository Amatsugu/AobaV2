using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;

using SixLabors.ImageSharp;

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
	ExtraLarge = 1024,
	ResponsiveSmall = 200,
	ResponsiveMedium = 450,
	ResponsiveLarge = 800
}

public static class ThumbnailSizeExt{
	extension (ThumbnailSize size)
	{
		public (int width, int height) GetThumbnailSize(Size? mediaSize = null)
		{
			if(mediaSize is Size srcSize)
			{
				var ratio = srcSize.Height / (float)srcSize.Width;

				var w = (int)size;
				var h = (int)(w * ratio);
				return (w, h);
			}
			else
				return ((int)size, (int)size);

		} 
	}
}
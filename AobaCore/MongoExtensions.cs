using AobaCore.Models;

using MongoDB.Bson;
using MongoDB.Driver;

using System;
using System.Collections.Generic;
using System.Text;

namespace AobaCore;

internal static class MongoExtensions
{
	public static FilterDefinition<Media> WithUserFilter(this FilterDefinition<Media> filter, ObjectId userId)
	{
		if (userId != default)
		{
			filter = Builders<Media>.Filter.And(Builders<Media>.Filter.Eq(m => m.Owner, userId), filter);
		}
		return filter;
	}

	public static FilterDefinition<Media> IdFilter(this IMongoCollection<Media> _collection, ObjectId mediaId)
	{
		return Builders<Media>.Filter.Eq(m => m.MediaId, mediaId);
	}

	public static FilterDefinition<Media> IdFilter(this IMongoCollection<Media> _collection, IEnumerable<ObjectId> mediaIds)
	{
		return Builders<Media>.Filter.In(m => m.MediaId, mediaIds);
	}
}

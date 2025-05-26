using AobaCore.Models;

using MongoDB.Bson;
using MongoDB.Driver;
using MongoDB.Driver.GridFS;

using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AobaCore.Services;
internal class ThumbnailService(IMongoDatabase db, AobaService aobaService)
{
	private readonly GridFSBucket _gridfs = new GridFSBucket(db);
	private readonly IMongoCollection<MeidaThumbnail> _thumbnails = db.GetCollection<MeidaThumbnail>("thumbs");

	public async Task<MemoryStream> GetThumbnailAsync(ObjectId id)
	{

	}

	public async Task GenerateThumbnailAsync(ObjectId id)
	{

	}
}

using AobaV2.Models;

using MongoDB.Bson;
using MongoDB.Driver;
using MongoDB.Driver.GridFS;

using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AobaCore;
public class MediaService(IMongoDatabase db, AobaService aobaService)
{
	private readonly GridFSBucket _gridFs = new(db);

	public async Task<Maybe<Media>> UploadMediaAsync(Stream data, string filename, ObjectId owner, CancellationToken cancellationToken = default)
	{
		var fileId = await _gridFs.UploadFromStreamAsync(filename, data, cancellationToken: cancellationToken);
		var media = new Media(fileId, filename,	owner);
		await aobaService.AddMediaAsync(media);
		return media;
	}
}

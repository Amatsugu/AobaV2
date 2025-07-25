﻿using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;

using SixLabors.ImageSharp;

namespace AobaCore.Models;

[BsonIgnoreExtraElements]
public class Media
{
	[BsonId]
	public ObjectId LegacyId { get; set; }
	public ObjectId MediaId { get; set; }
	public string Filename { get; set; }
	public MediaType MediaType { get; set; }
	public string Ext { get; set; }
	public int ViewCount { get; set; }
	public ObjectId Owner { get; set; }
	public DateTime UploadDate { get; set; }
	public string[] Tags { get; set; } = [];
	public Size? Dimensions { get; set; }
	public Dictionary<ThumbnailSize, ObjectId> Thumbnails { get; set; } = [];


	public static readonly Dictionary<string, MediaType> KnownTypes = new()
		{
			{ ".jpg", MediaType.Image },
			{ ".jpeg", MediaType.Image },
			{ ".jxr", MediaType.Image },
			{ ".avif", MediaType.Image },
			{ ".png", MediaType.Image },
			{ ".apng", MediaType.Image },
			{ ".webp", MediaType.Image },
			{ ".qoi", MediaType.Image },
			{ ".ico", MediaType.Image },
			{ ".gif", MediaType.Image },
			{ ".mp3", MediaType.Audio },
			{ ".flac", MediaType.Audio },
			{ ".alac", MediaType.Audio },
			{ ".mp4", MediaType.Video },
			{ ".m4v", MediaType.Video },
			{ ".webm", MediaType.Video },
			{ ".mov", MediaType.Video },
			{ ".avi", MediaType.Video },
			{ ".mkv", MediaType.Video },
			{ ".txt", MediaType.Text },
			{ ".log", MediaType.Text },
			{ ".css", MediaType.Code },
			{ ".cs", MediaType.Code },
			{ ".cpp", MediaType.Code },
			{ ".lua", MediaType.Code },
			{ ".js", MediaType.Code },
			{ ".htm", MediaType.Code },
			{ ".html", MediaType.Code },
			{ ".cshtml", MediaType.Code },
			{ ".xml", MediaType.Code },
			{ ".json", MediaType.Code },
			{ ".py", MediaType.Code },
			{ ".rs", MediaType.Code },
			{ ".zed", MediaType.Code },
			{ ".ts", MediaType.Code },
			{ ".astro", MediaType.Code },
		};

	[BsonConstructor]
	private Media()
	{
		Filename = string.Empty;
		Ext = string.Empty;
	}

	public Media(ObjectId fileId, string filename, ObjectId owner)
	{
		MediaType = GetMediaType(filename);
		Ext = Path.GetExtension(filename);
		Filename = filename;
		MediaId = fileId;
		Owner = owner;
		LegacyId = ObjectId.GenerateNewId();
		UploadDate = DateTime.UtcNow;
		Tags = DeriveTags(filename);
	}

	public string GetMediaUrl()
	{
		return this switch
		{
			Media { MediaType: MediaType.Raw or MediaType.Text or MediaType.Code } => $"/m/{MediaId}/{Uri.EscapeDataString(Filename)}",
			_ => $"/m/{MediaId}"
		};
	}

	public static MediaType GetMediaType(string filename)
	{
		string ext = Path.GetExtension(filename);
		if (KnownTypes.TryGetValue(ext, out MediaType mType))
			return mType;
		else
			return MediaType.Raw;
	}

	public static string[] DeriveTags(string filename)
	{
		return filename.Split('_')
			.SelectMany(v => v.Split('-'))
			.SelectMany(v => v.Split(' '))
			.ToArray();
	}
}

public enum MediaType
{
	Image,
	Audio,
	Video,
	Text,
	Code,
	Raw
}
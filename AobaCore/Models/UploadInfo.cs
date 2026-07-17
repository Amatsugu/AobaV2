using MongoDB.Bson;

public record UploadInfo(ObjectId Id, string Url, string ContentType);

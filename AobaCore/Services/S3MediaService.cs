using System.Collections.Concurrent;
using Amazon.S3;
using Amazon.S3.Model;
using HeyRed.Mime;
using MaybeError.Errors;
using Microsoft.Extensions.Configuration;
using MongoDB.Bson;

namespace AobaCore.Services;

public class S3MediaService
{
	private readonly string? _bucket;
	private readonly AmazonS3Client _client;
	private readonly ConcurrentDictionary<ObjectId, string> _pendingUploads = [];
	public S3MediaService(IConfiguration config)
	{
		var cfg = new AmazonS3Config
		{
			ServiceURL = config["S3_URL"] ?? throw new NullReferenceException("S3_URL is not set"),
			ForcePathStyle = true
		};
		var access = config["AWS_ACCESS_KEY_ID"] ?? throw new NullReferenceException("AWS_ACCESS_KEY_ID key is not set");
		var secret = config["AWS_SECRET_ACCESS_KEY"] ?? throw new NullReferenceException("AWS_SECRET_ACCESS_KEY key is not set");
		_bucket = config["S3_BUCKET"] ?? "aoba";
		_client = new AmazonS3Client(access, secret, cfg);
	}

	public async Task<MaybeEx<string, AmazonS3Exception>> UploadFileAsync(string filename, string mimeType, Stream data, CancellationToken cancellationToken = default)
	{
		try
		{
			var request = new PutObjectRequest
			{
				BucketName = _bucket,
				Key = filename,
				InputStream = data,
				AutoCloseStream = true,
				ContentType = mimeType,
			};

			await _client.PutObjectAsync(request, cancellationToken);
			return filename;
		}
		catch (AmazonS3Exception e)
		{
			return e;
		}
	}

	public async Task DeleteFileAsync(string filename, CancellationToken cancellationToken = default)
	{
		await _client.DeleteObjectAsync(new DeleteObjectRequest
		{
			BucketName = _bucket,
			Key = filename,
		});
	}

	public async Task<MaybeEx<bool, AmazonS3Exception>> FileExistsAsync(string filename, CancellationToken cancellationToken = default)
	{
		try
		{
			await _client.GetObjectMetadataAsync(_bucket, filename, cancellationToken);
			return true;
		}
		catch (AmazonS3Exception ex) when (ex.StatusCode == System.Net.HttpStatusCode.NotFound)
		{
			return false;
		}
		catch (AmazonS3Exception e)
		{
			return e;
		}
	}

	public async Task<MaybeEx<Stream, AmazonS3Exception>> GetFileAsync(string filename, CancellationToken cancellationToken = default)
	{
		try
		{

			var result = await _client.GetObjectAsync(_bucket, filename, cancellationToken);
			return result.ResponseStream;
		}
		catch (AmazonS3Exception e)
		{
			return e;
		}
	}

	public Maybe<UploadInfo> CreateUploadUrl(string filename)
	{
		var ext = Path.GetExtension(filename);
		var id = ObjectId.GenerateNewId();
		var sign = new GetPreSignedUrlRequest
		{
			BucketName = _bucket,
			ContentType = MimeTypesMap.GetMimeType(filename),
			Expires = DateTime.UtcNow.AddMinutes(5),
			Key = $"pending/{id}{ext}",
			Verb = HttpVerb.PUT
		};
		var url = _client.GetPreSignedURL(sign);
		if (!_pendingUploads.TryAdd(id, filename))
			return new Error("Failed to prepare upload url", "Failed to add to pending dict");

		return new UploadInfo(id, url, sign.ContentType);
	}

	public async Task<Maybe<(string filename, string cdnUrl)>> CompleteUploadAsync(ObjectId id, CancellationToken cancellationToken = default)
	{
		if (!_pendingUploads.TryRemove(id, out var filename))
			return new Error("No Pending Upload exists with that id");
		var ext = Path.GetExtension(filename);
		var srcKey = $"pending/{id}{ext}";
		var dstKey = $"{id}{ext}";

		try
		{

			var req = new CopyObjectRequest
			{
				SourceBucket = _bucket,
				DestinationBucket = _bucket,
				SourceKey = srcKey,
				DestinationKey = dstKey
			};

			await _client.CopyObjectAsync(req, cancellationToken);

			await DeleteFileAsync(srcKey, cancellationToken);
			return (filename, dstKey);
		}
		catch (Exception ex)
		{
			return new ExceptionError(ex);
		}
	}
}

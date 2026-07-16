using Amazon.S3;
using Amazon.S3.Model;

using Microsoft.Extensions.Configuration;

namespace AobaCore.Services;

public class S3MediaService
{
	private readonly string? _bucket;
	private readonly AmazonS3Client _client;
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
		}catch(AmazonS3Exception e)
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
		catch (AmazonS3Exception ex) when(ex.StatusCode == System.Net.HttpStatusCode.NotFound)
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
}

using Aoba.RPC;

using AobaCore.Services;

using AobaServer.Models;
using AobaServer.Utils;

using Google.Protobuf.WellKnownTypes;

using Grpc.Core;
using HeyRed.Mime;
using System.Text.Json;

namespace AobaServer.Services;

public class AobaRpcService(AobaService aobaService, ThumbnailService thumbnailService, AccountsService accountsService, AuthConfigService authConfig, HostInfo host, S3MediaService s3) : AobaRpc.AobaRpcBase
{

	public override async Task<MediaResponse> GetMedia(Id request, ServerCallContext context)
	{
		var media = await aobaService.GetMediaAsync(request.ToObjectId(), context.CancellationToken);
		return media.ToResponse(host);
	}

	public override async Task<ListResponse> ListMedia(PageFilter request, ServerCallContext context)
	{
		var user = context.GetUserId();
		var result = await aobaService.FindMediaAsync(request.Query, user, request.HasPage ? request.Page : 1, request.HasPageSize ? request.PageSize : 100);
		return result.ToResponse(host);
	}

	public override async Task<Empty> SetMediaClass(SetMediaClassRequest request, ServerCallContext context)
	{
		await aobaService.SetMediaClassAsync(request.Id.ToObjectId(), (AobaCore.Models.MediaClass)request.Class, context.CancellationToken);
		return new Empty();
	}

	public override async Task<ShareXResponse> GetShareXDestination(Empty request, ServerCallContext context)
	{
		var userId = context.GetHttpContext().User.GetId();
		var user = await accountsService.GetUserAsync(userId, context.CancellationToken);
		if (user == null)
			return new ShareXResponse { Error = "User does not exist" };
		var authInfo = await authConfig.GetDefaultAuthInfoAsync();
		var token = user.GetToken(authInfo);
		var dest = new ShareXDestination
		{
			DeletionURL = string.Empty,
			ThumbnailURL = string.Empty,
			Headers = new()
			{
				{ "Authorization", $"Bearer {token}" }
			}
		};
		return new ShareXResponse
		{
			Destination = JsonSerializer.Serialize(dest)
		};
	}

	public override async Task<Empty> DeleteMedia(Id request, ServerCallContext context)
	{
		var media = await aobaService.GetMediaAsync(request.ToObjectId());
		if (media == null)
			return new Empty();
		if (media.Cdn != null)
		{
			await s3.DeleteFileAsync(media.Cdn.Url, CancellationToken.None);
			foreach (var (_, key) in media.Cdn.ThumbnailUrls)
			{
				await s3.DeleteFileAsync(key, CancellationToken.None);
			}
			await aobaService.DeleteMediaAsync(media.MediaId, context.CancellationToken);
		}
		else
		{
			await aobaService.DeleteFileAsync(media.MediaId, context.CancellationToken);
			foreach (var (_, id) in media.Thumbnails)
			{
				await thumbnailService.DeleteThumbnailDirectAsync(id);
			}
		}
		return new Empty();
	}

	public override async Task<Empty> DeleteMediaBulk(IdList request, ServerCallContext context)
	{
		var media = await aobaService.GetMediaAsync(request.ToObjectId(), context.CancellationToken);
		if (media.Count == 0)
			return new Empty();
		await aobaService.DeleteFilesAsync(request.ToObjectId(), context.CancellationToken);
		foreach (var item in media)
		{
			if (item.Cdn != null)
			{
				await s3.DeleteFileAsync(item.Cdn.Url, CancellationToken.None);
				foreach (var (_, key) in item.Cdn.ThumbnailUrls)
				{
					await s3.DeleteFileAsync(key, CancellationToken.None);
				}
				await aobaService.DeleteMediaAsync(item.MediaId, context.CancellationToken);
			}
			else
			{
				foreach (var (_, id) in item.Thumbnails)
				{
					await thumbnailService.DeleteThumbnailDirectAsync(id);
				}
			}
		}
		return new Empty();
	}

	public override async Task<Empty> SetMediaClassBulk(SetMediaClassBulkRequest request, ServerCallContext context)
	{
		await aobaService.SetMediaClassAsync(request.Ids.ToObjectId(), (AobaCore.Models.MediaClass)request.Class, context.CancellationToken);
		return new Empty();
	}

	public override Task<UploadTargetResponse> StartUpload(UploadRequest request, ServerCallContext context)
	{
		var response = new UploadTargetResponse();
		foreach(var file in request.Files){
			var info = s3.CreateUploadUrl(file.Filename);
			if (info.HasError)
			{
				return Task.FromResult(new UploadTargetResponse
				{
					Error = info.Error
				});
			}
			var tgt = new UploadTarget
			{
				ContentType = MimeTypesMap.GetMimeType(file.Filename),
				Id = info.Value.Id.ToId(),
				SignedUrl = info.Value.Url
			};
			response.Targets.Targets.Add(tgt);
		}
		return Task.FromResult(response);
	}

	public override async Task<UploadResult> CompleteUpload(Id id, ServerCallContext context)
	{
		var file = await s3.CompleteUploadAsync(id.ToObjectId());
		if (file.HasError)
		{
			return new UploadResult
			{
				ErrorMessage = file.Error.ToString()
			};
		}
		var media = new AobaCore.Models.Media(id.ToObjectId(), file.Value.filename, context.GetHttpContext().User.GetId())
		{
			Cdn = new()
			{
				Url = file.Value.cdnUrl,
			}
		};

		await aobaService.AddMediaAsync(media, context.CancellationToken);

		return new UploadResult
		{
			Media = media.ToMediaModel(host)
		};
	}
}

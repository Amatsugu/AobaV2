using Aoba.RPC;

using AobaCore.Services;

using AobaServer.Models;
using AobaServer.Utils;

using Google.Protobuf.WellKnownTypes;

using Grpc.Core;

using System.Text.Json;

namespace AobaServer.Services;

public class AobaRpcService(AobaService aobaService, ThumbnailService thumbnailService, AccountsService accountsService, AuthConfigService authConfig) : AobaRpc.AobaRpcBase
{
	public override async Task<MediaResponse> GetMedia(Id request, ServerCallContext context)
	{
		var media = await aobaService.GetMediaAsync(request.ToObjectId(), context.CancellationToken);
		return media.ToResponse();
	}

	public override async Task<ListResponse> ListMedia(PageFilter request, ServerCallContext context)
	{
		var user = context.GetUserId();
		var result = await aobaService.FindMediaAsync(request.Query, user, request.HasPage ? request.Page : 1, request.HasPageSize ? request.PageSize : 100);
		return result.ToResponse();
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
		await aobaService.DeleteFileAsync(media.MediaId, context.CancellationToken);
		foreach (var (_, id) in media.Thumbnails)
		{
			await thumbnailService.DeleteThumbnailDirectAsync(id);
		}
		return new Empty();
	}

	public override async Task<Empty> DeleteMediaBulk(IdList request, ServerCallContext context)
	{
		var media = await aobaService.GetMediaAsync(request.ToObjectId(), context.CancellationToken);
		if(media.Count == 0)
			return new Empty();
		await aobaService.DeleteFilesAsync(request.ToObjectId(), context.CancellationToken);
		foreach (var item in media)
		{
			foreach (var (_, id) in item.Thumbnails)
			{
				await thumbnailService.DeleteThumbnailDirectAsync(id);
			}
		}
		return new Empty();
	}
}
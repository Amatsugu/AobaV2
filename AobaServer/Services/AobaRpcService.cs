using Aoba.RPC;

using AobaCore.Services;

using AobaServer.Models;
using AobaServer.Utils;

using Google.Protobuf.WellKnownTypes;

using Grpc.Core;

using MongoDB.Bson.IO;

using System.Text.Json;
using System.Text.Json.Serialization;

namespace AobaServer.Services;

public class AobaRpcService(AobaService aobaService, AccountsService accountsService, AuthInfo authInfo) : AobaRpc.AobaRpcBase
{
	public override async Task<MediaResponse> GetMedia(Id request, ServerCallContext context)
	{
		var media = await aobaService.GetMediaFromLegacyIdAsync(request.ToObjectId());
		return media.ToResponse();
	}

	public override async Task<ListResponse> ListMedia(PageFilter request, ServerCallContext context)
	{
		var user = context.GetUserId();
		var result = await aobaService.FindMediaAsync(request.Query, user, request.HasPage ? request.Page : 1, request.HasPageSize ? request.PageSize : 100);
		return result.ToResponse();
	}

	public override async Task<ShareXResponse> GetShareXDestination(Empty request, ServerCallContext context)
	{
		var userId = context.GetHttpContext().User.GetId();
		var user = await accountsService.GetUserAsync(userId, context.CancellationToken);
		if (user == null)
			return new ShareXResponse { Error = "User does not exist" };
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
			Destination = JsonSerializer.Serialize(dest, new JsonSerializerOptions
			{
				WriteIndented = true
			})
		};
	}

}
using AobaCore;

using Aoba.RPC;

using AobaServer.Utils;

using Grpc.Core;

namespace AobaServer.Services;

public class AobaRpcService(AobaService aobaService) : AobaRpc.AobaRpcBase
{
	public override async Task<MediaResponse> GetMedia(Id request, ServerCallContext context)
	{
		var media = await aobaService.GetMediaAsync(request.ToObjectId());
		return media.ToResponse();
	}

	public override async Task<ListResponse> ListMedia(PageFilter request, ServerCallContext context)
	{
		var result = await aobaService.FindMediaAsync(request.Query, request.HasPage ? request.Page : 1, request.HasPageSize ? request.PageSize : 100);
		return result.ToResponse();
	}

}

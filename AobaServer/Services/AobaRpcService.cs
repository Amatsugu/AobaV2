using AobaCore;

using Grpc.Core;

namespace AobaServer.Services;

public class AobaRpcService(AobaService aobaService) : AobaRPC.AobaRPCBase
{
	public override Task<MediaModel> GetMedia(Id request, ServerCallContext context)
	{
		return base.GetMedia(request, context);
	}
}

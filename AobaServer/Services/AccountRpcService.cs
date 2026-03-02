using Aoba.RPC;
using Aoba.RPC.Account;

using Google.Protobuf.WellKnownTypes;

using Grpc.Core;

namespace AobaServer.Services;

public class AccountRpcService : AccountRpc.AccountRpcBase
{
	public override Task<PasskeyRegistrationCreds> RegisterPasskey(Empty request, ServerCallContext context)
	{
		return base.RegisterPasskey(request, context);
	}

	public override Task<Empty> CompletePasskeyRegistration(PasskeyPublicKey request, ServerCallContext context)
	{
		return base.CompletePasskeyRegistration(request, context);
	}

}

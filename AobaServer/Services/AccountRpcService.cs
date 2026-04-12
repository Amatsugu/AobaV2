using Aoba.RPC;
using Aoba.RPC.Account;

using AobaCore.Services;

using AobaServer.Utils;

using Fido2NetLib;

using Google.Protobuf.WellKnownTypes;

using Grpc.Core;

using Isopoh.Cryptography.Argon2;

namespace AobaServer.Services;

public class AccountRpcService(IFido2 fido2, AccountsService accounts) : AccountRpc.AccountRpcBase
{
	public override async Task<PasskeyCredentialCreateOptions> RegisterPasskey(Empty request, ServerCallContext context)
	{
		var curUser = await accounts.GetUserAsync(context.GetUserId(), context.CancellationToken);
		if (curUser == null)
			throw new Exception($"Logged in user does not exist somehow. Id: {context.GetUserId()}");
		var user = new Fido2User
		{
			DisplayName = curUser.Username,
			Id = curUser.Id.ToByteArray(),
			Name = curUser.Username
		};

		var credOptions = fido2.RequestNewCredential(new RequestNewCredentialParams
		{
			User = user,
			ExcludeCredentials = curUser.CredentialDescriptors,
			AuthenticatorSelection = new AuthenticatorSelection
			{
				ResidentKey = Fido2NetLib.Objects.ResidentKeyRequirement.Required,
				UserVerification = Fido2NetLib.Objects.UserVerificationRequirement.Preferred
			}
		});
		

		return credOptions.ToRPC();
	}

	public override Task<Empty> CompletePasskeyRegistration(PasskeyRegistrationCredentials request, ServerCallContext context)
	{
		return base.CompletePasskeyRegistration(request, context);
	}

}

using Aoba.RPC;
using Aoba.RPC.Account;

using AobaCore.Services;

using AobaServer.Utils;

using Fido2NetLib;

using Google.Protobuf.WellKnownTypes;

using Grpc.Core;

using Isopoh.Cryptography.Argon2;

namespace AobaServer.Services;

public class AccountRpcService(IFido2 fido2, AccountsService accounts, PasskeyCreationOptsCache optsCache) : AccountRpc.AccountRpcBase
{
	public override async Task<PasskeyCreationResponse> RegisterPasskey(Empty request, ServerCallContext context)
	{
		var user = await accounts.GetUserAsync(context.GetUserId());
		if (user == null)
		{
			return new PasskeyCreationResponse
			{
				Error = new LoginError
				{
					Message = "User is not logged in or does not exist"
				}
			};
		}
		var opts = fido2.RequestNewCredential(new RequestNewCredentialParams
		{
			User = new Fido2User
			{
				DisplayName = user.Username,
				Id = user.Id.ToByteArray(),
				Name = user.Username,
			},
			AuthenticatorSelection = new AuthenticatorSelection
			{
				ResidentKey = Fido2NetLib.Objects.ResidentKeyRequirement.Required,
				UserVerification = Fido2NetLib.Objects.UserVerificationRequirement.Required
			},
			ExcludeCredentials = user.GetCredentialDescriptors(),
			PubKeyCredParams = [
				Fido2NetLib.PubKeyCredParam.ES256,
				Fido2NetLib.PubKeyCredParam.RS256
			],
			AttestationPreference = Fido2NetLib.Objects.AttestationConveyancePreference.None
		});
		optsCache.AddOrUpdate(user.Id, opts, (_, _) => opts);
		return opts.ToResponse();
	}

	public async override Task<Empty> CompletePasskeyRegistration(PasskeyRegistrationCredentials request, ServerCallContext context)
	{
		if (!optsCache.TryRemove(context.GetUserId(), out var opts))
			return new Empty();

		var cred = await fido2.MakeNewCredentialAsync(new MakeNewCredentialParams
		{
			AttestationResponse = new AuthenticatorAttestationRawResponse
			{
				Id = request.Id,
				RawId = [.. request.RawId],
				Response = new AuthenticatorAttestationRawResponse.AttestationResponse
				{
					ClientDataJson = [..request.ClientDataJson],
					AttestationObject = [..request.AttestationObject],
					Transports = []
				},
			},
			OriginalOptions = opts,
			IsCredentialIdUniqueToUserCallback = async (usr, ct) => ! await accounts.CredentialExistsAsync(usr.CredentialId, ct)
		}, context.CancellationToken);


		await accounts.StoreCredentialsAsync("Passkey", cred, context.CancellationToken);
		return new Empty();
	}

}

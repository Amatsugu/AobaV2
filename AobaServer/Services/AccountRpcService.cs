using System.Diagnostics;
using Aoba.RPC;
using Aoba.RPC.Account;
using Aoba.RPC.Models;
using AobaCore;
using AobaCore.Services;

using AobaServer.Utils;

using Fido2NetLib;

using Google.Protobuf.WellKnownTypes;

using Grpc.Core;


namespace AobaServer.Services;

public class AccountRpcService(IFido2 fido2, AccountsService accounts, PasskeyCreationOptsCache optsCache, UserSessionsService sessionsService, ILogger<AccountRpcService> logger) : AccountRpc.AccountRpcBase
{
	public override async Task<PasskeyCreationResponse> RegisterPasskey(Empty request, ServerCallContext context)
	{
		var user = await accounts.GetUserAsync(context.GetUserId());
		if (user == null)
		{
			return new PasskeyCreationResponse
			{
				Error = new Error
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
		if (!optsCache.Add(user.Id, opts))
		{
			logger.LogWarning("Failed to cache {name}, id conflict", nameof(CredentialCreateOptions));
			return new PasskeyCreationResponse
			{
				Error = new Error
				{
					Message = "Failed to start passkey registration"
				}
			};
		}
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
					ClientDataJson = [.. request.ClientDataJson],
					AttestationObject = [.. request.AttestationObject],
					Transports = []
				},
			},
			OriginalOptions = opts,
			IsCredentialIdUniqueToUserCallback = async (usr, ct) => !await accounts.CredentialExistsAsync(usr.CredentialId, ct)
		}, context.CancellationToken);


		await accounts.StoreCredentialsAsync("Passkey", cred, context.CancellationToken);
		return new Empty();
	}

	public async override Task<UserSessionListResponse> GetUserSessions(Empty request, ServerCallContext context)
	{
		var sessions = await sessionsService.GetUserSessionsAsync(context.GetUserId(), context.CancellationToken);
		var list = new UserSessionList();
		list.Sessions.AddRange(sessions.ToRpc(context.GetTokenId()));
		return new UserSessionListResponse
		{
			List = list
		};
	}

	public async override Task<LoginResponse> RefreshSession(Empty request, ServerCallContext context)
	{
		var authConfig = context.GetHttpContext().RequestServices.GetRequiredService<AuthConfigService>();
		var authInfo = await authConfig.GetDefaultAuthInfoAsync();
		var user = await accounts.GetUserAsync(context.GetUserId());
		if (user == null)
			return new LoginResponse
			{
				Error = new Error
				{
					Message = "User does not exist"
				}
			};
		var session = await sessionsService.RefreshSession(context.GetUserId(), context.GetTokenId(), tokenId => user.GetToken(authInfo, tokenId));
		if (session == null)
			return new LoginResponse
			{
				Error = new Error
				{
					Message = "Failed to renew session"
				}
			};
		return new LoginResponse
		{
			Jwt = new Jwt
			{
				Token = session.Token,
				Exp = session.Expiration.UnixTime
			}
		};
	}

	public async override Task<Empty> RevokeSession(Id request, ServerCallContext context)
	{
		await sessionsService.RevokeUserSessionAsync(context.GetUserId(), request.ToObjectId(), context.CancellationToken);
		return new Empty();
	}

}

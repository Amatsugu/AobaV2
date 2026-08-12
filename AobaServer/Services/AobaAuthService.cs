using Aoba.RPC.Auth;

using AobaCore.Models;
using AobaCore.Services;

using AobaServer.Models;
using AobaServer.Utils;

using Grpc.Core;

using Microsoft.AspNetCore.Authorization;
using Aoba.RPC;
using Fido2NetLib;
using MongoDB.Bson;
using Google.Protobuf.WellKnownTypes;


namespace AobaServer.Services;

public class AobaAuthService(AccountsService accountsService, AuthConfigService authConfig, IFido2 fido2, PasskeyAssertionOptsCache optsCache, ILogger<AobaAuthService> logger) : AuthRpc.AuthRpcBase
{
	[AllowAnonymous]
	public override async Task<LoginResponse> Login(Credentials request, ServerCallContext context)
	{
		var user = await accountsService.VerifyLoginAsync(request.User, request.Password, context.CancellationToken);
		if (user == null)
			return new LoginResponse
			{
				Error = new LoginError
				{
					Message = "Invalid login credentials"
				}
			};
		var authInfo = await authConfig.GetDefaultAuthInfoAsync();
		var token = user.GetToken(authInfo);
		return new LoginResponse
		{
			Jwt = new ()
			{
				Token = token
			}
		};
	}

	public override Task<PasskeyAssertionResponse> GetAssertionOptions(Empty request, ServerCallContext context)
	{
		var ceremonyId = ObjectId.GenerateNewId();
		logger.LogInformation("Starting Passkey assertion: {id}", ceremonyId);
		var opts = fido2.GetAssertionOptions(new GetAssertionOptionsParams
		{
			AllowedCredentials = [],
			UserVerification = Fido2NetLib.Objects.UserVerificationRequirement.Required
		});
		if (!optsCache.TryAdd(ceremonyId, opts))
			return Task.FromResult(new PasskeyAssertionResponse { ErrorMessage = "Failed to get assertion options" });

		return Task.FromResult(opts.ToResponse(ceremonyId));
	}

	public override async Task<LoginResponse> LoginPasskey(PasskeyLoginRequest request, ServerCallContext context)
	{
		var existingCred = await accountsService.GetStoredCredentialAsync([..request.RawId]);

		if (existingCred == null || !optsCache.TryRemove(request.CeremonyId.ToObjectId(), out var opts))
			return new LoginResponse
			{
				Error = new LoginError { Message = "Invalid credentials" }
			};
		logger.LogInformation("Assertion Response for {id}", request.CeremonyId.ToObjectId());

		var result = await fido2.MakeAssertionAsync(new MakeAssertionParams
		{
			AssertionResponse = new AuthenticatorAssertionRawResponse
			{
				Id 	= request.Id,
				RawId = [..request.RawId],
				Response = new AuthenticatorAssertionRawResponse.AssertionResponse
				{
					AuthenticatorData = [..request.AuthenticatorData],
					ClientDataJson = [..request.ClientDataJson],
					Signature = [..request.Signature],
					UserHandle = [.. request.UserHandle]
				},
				Type = Fido2NetLib.Objects.PublicKeyCredentialType.PublicKey
			},
			StoredPublicKey = existingCred.PublicKey,
			StoredSignatureCounter = existingCred.Counter,
			OriginalOptions = opts,
			IsUserHandleOwnerOfCredentialIdCallback = (usr, ct) => accountsService.UserOwnsCredentialAsync(new ObjectId(usr.UserHandle), usr.CredentialId, ct)
		}, context.CancellationToken);


		var userId = new ObjectId([..request.UserHandle]);
		var user = await accountsService.VerifyPasskeyLoginAsync(userId, result, context.CancellationToken);

		if(user == null)
			return new LoginResponse
			{
				Error = new LoginError { Message = "Invalid credentials" }
			};

		var authInfo = await authConfig.GetDefaultAuthInfoAsync();
		var token = user.GetToken(authInfo);
		return new LoginResponse
		{
			Jwt = new()
			{
				Token = token
			}
		};
	}


}
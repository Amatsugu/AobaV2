using Aoba.RPC.Auth;

using AobaCore.Models;
using AobaCore.Services;

using AobaServer.Models;
using AobaServer.Utils;

using Grpc.Core;

using Microsoft.AspNetCore.Authorization;
using Aoba.RPC;


namespace AobaServer.Services;

public class AobaAuthService(AccountsService accountsService, AuthInfo authInfo) : AuthRpc.AuthRpcBase
{
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
		var token = user.GetToken(authInfo);
		return new LoginResponse
		{
			Jwt = new ()
			{
				Token = token
			}
		};
	}


}
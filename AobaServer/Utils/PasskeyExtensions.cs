using Aoba.RPC;

using Isopoh.Cryptography.Argon2;

namespace AobaServer.Utils;

public static class PasskeyExtensions
{
	public static PublicKeyCredentialRpEntity ToRPC(this Fido2NetLib.PublicKeyCredentialRpEntity value) 
	{
		return new PublicKeyCredentialRpEntity
		{
			Id = value.Id,
			Icon = value.Icon,
			Name = value.Name,
		};
	}

	public static PublicKeyCredentialUser ToRPC(this Fido2NetLib.Fido2User value)
	{
		return new PublicKeyCredentialUser
		{
			Id = value.Id.ToB64String(),
			DisplayName = value.DisplayName,
			Name = value.Name,
		};
	}

	public static PubKeyCredParam ToRPC(this Fido2NetLib.PubKeyCredParam value)
	{
		return new PubKeyCredParam
		{
			Alg = value.Alg.ToString(),
			Type = value.Type.ToString(),
		};
	}

	public static IEnumerable<PubKeyCredParam> ToRPC(this IEnumerable<Fido2NetLib.PubKeyCredParam> value)
	{
		return value.Select(x => x.ToRPC());
	}

	public static PasskeyCredentialCreateOptions ToRPC(this Fido2NetLib.CredentialCreateOptions value)
	{
		var opts = new PasskeyCredentialCreateOptions
		{
			Challenge = value.Challenge.ToB64String(),
			Rp = value.Rp.ToRPC(),
			User = value.User.ToRPC()
		};
		//todo: excluded credentials
		opts.PubKeyParams.AddRange(value.PubKeyCredParams.ToRPC());
		return opts;
	}
}

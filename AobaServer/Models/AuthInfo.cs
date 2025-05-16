using MongoDB.Bson.IO;

using System.Security.Cryptography;
using System.Text.Json;

namespace AobaServer.Models;

public class AuthInfo
{
	public required string Issuer { get; set; }
	public required string Audience { get; set; }
	public required byte[] SecureKey { get; set; }

	/// <summary>
	/// Save this auth into in a json format to the sepcified file
	/// </summary>
	/// <param name="path">File path</param>
	/// <returns></returns>
	public AuthInfo Save(string path)
	{
		File.WriteAllText(path, JsonSerializer.Serialize(this));
		return this;
	}

	/// <summary>
	/// Generate a new Auth Info with newly generated keys
	/// </summary>
	/// <param name="issuer"></param>
	/// <param name="audience"></param>
	/// <returns></returns>
	public static AuthInfo Create(string issuer, string audience)
	{
		var auth = new AuthInfo
		{
			Issuer = issuer,
			Audience = audience,
			SecureKey = GenetateJWTKey()
		};
		return auth;
	}

	/// <summary>
	/// Load auth info from a json file
	/// </summary>
	/// <param name="path">File path</param>
	/// <returns></returns>
	internal static AuthInfo? Load(string path)
	{
		return JsonSerializer.Deserialize<AuthInfo>(File.ReadAllText(path));
	}

	internal static AuthInfo LoadOrCreate(string path, string issuer, string audience)
	{
		if (File.Exists(path))
		{
			var loaded = Load(path);
			if (loaded != null)
				return loaded;
		}
		var info = Create(issuer, audience);
		info.Save(path);
		return info;
	}

	/// <summary>
	/// Generate a new key for use by JWT
	/// </summary>
	/// <returns></returns>
	public static byte[] GenetateJWTKey(int size = 64)
	{
		var key = new byte[size];
		RandomNumberGenerator.Fill(key);
		return key;
	}
}
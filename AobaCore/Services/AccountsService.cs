using AobaCore.Models;

using Fido2NetLib.Objects;

using Isopoh.Cryptography.Argon2;

using MongoDB.Bson;
using MongoDB.Driver;

using System;
using System.Collections.Generic;
using System.Linq;
using System.Security.Cryptography;
using System.Text;
using System.Threading.Tasks;

namespace AobaCore.Services;
public class AccountsService(IMongoDatabase db)
{
	public readonly IMongoCollection<User> _users = db.GetCollection<User>("users");

	public async Task<User?> GetUserAsync(ObjectId id, CancellationToken cancellationToken = default)
	{
		return await _users.Find(u => u.Id == id).FirstOrDefaultAsync(cancellationToken);
	}

	public async Task<User?> VerifyLoginAsync(string username, string password, CancellationToken cancellationToken = default)
	{
		var user = await _users.Find(u => u.Username == username).FirstOrDefaultAsync(cancellationToken);
		if(user == null)
			return null;

		if(user.IsArgon && Argon2.Verify(user.PasswordHash, password))
			return user;

		if(LegacyVerifyPassword( password, user.PasswordHash))
		{
#if !DEBUG
			var argon2Hash = Argon2.Hash(password);
			var update = Builders<User>.Update.Set(u => u.PasswordHash, argon2Hash).Set(u => u.IsArgon, true);
			await _users.UpdateOneAsync(u => u.Id == user.Id, update, cancellationToken: cancellationToken);
#endif
			return user;
		}

		return null;
	}


	public static bool LegacyVerifyPassword(string password, string passwordHash)
	{
		if (string.IsNullOrWhiteSpace(password) || string.IsNullOrWhiteSpace(passwordHash))
			return false;
		/* Extract the bytes */
		byte[] hashBytes = Convert.FromBase64String(passwordHash);
		/* Get the salt */
		byte[] salt = new byte[16];
		Array.Copy(hashBytes, 0, salt, 0, 16);

		var hash= Rfc2898DeriveBytes.Pbkdf2(password, salt, 10000, HashAlgorithmName.SHA1, 20);
		/* Compare the results */
		for (int i = 0; i < 20; i++)
			if (hashBytes[i + 16] != hash[i])
				return false;
		return true;
	}

	public async Task<List<PublicKeyCredentialDescriptor>> GetPublicKeyCredentialDescriptorsAsync(ObjectId id, CancellationToken cancellationToken = default)
	{
		var creds = await _users.Find(u => u.Id == id).Project(u => u.CredentialDescriptors).FirstOrDefaultAsync(cancellationToken);
		return creds ?? [];
	}
}

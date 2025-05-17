using AobaCore.Models;

using Isopoh.Cryptography.Argon2;

using MongoDB.Bson;
using MongoDB.Driver;

using System;
using System.Collections.Generic;
using System.Linq;
using System.Security.Cryptography;
using System.Text;
using System.Threading.Tasks;

namespace AobaCore;
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
		/* Compute the hash on the password the user entered */
		var pbkdf2 = new Rfc2898DeriveBytes(password, salt, 10000, HashAlgorithmName.SHA1);
		byte[] hash = pbkdf2.GetBytes(20);
		/* Compare the results */
		for (int i = 0; i < 20; i++)
			if (hashBytes[i + 16] != hash[i])
				return false;
		return true;
	}
}

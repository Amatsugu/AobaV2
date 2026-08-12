using AobaCore.Models;

using Fido2NetLib.Objects;

using Isopoh.Cryptography.Argon2;

using MongoDB.Bson;
using MongoDB.Driver;
using MongoDB.Driver.Linq;

using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Security.Cryptography;
using System.Text;
using System.Threading.Tasks;

namespace AobaCore.Services;

public class AccountsService(IMongoDatabase db)
{
	private readonly IMongoCollection<User> _users = db.GetCollection<User>("users");

#if DEBUG
	public async Task CreateDevAccountAsync()
	{
		if (await _users.AsQueryable().AnyAsync())
			return;
		var user = new User
		{
			Username = "dev",
			IsArgon = true,
			Role = "admin",
			PasswordHash = Argon2.Hash("dev")
		};
		await _users.InsertOneAsync(user);
	}
#endif

	public async Task<User?> GetUserAsync(ObjectId id, CancellationToken cancellationToken = default)
	{
		return await _users.Find(u => u.Id == id).FirstOrDefaultAsync(cancellationToken);
	}

	public async Task<User?> VerifyLoginAsync(string username, string password, CancellationToken cancellationToken = default)
	{
		var user = await _users.Find(u => u.Username == username).FirstOrDefaultAsync(cancellationToken);
		if (user == null)
			return null;

		if (user.IsArgon && Argon2.Verify(user.PasswordHash, password))
			return user;

		if (LegacyVerifyPassword(password, user.PasswordHash))
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

	public Task<User?> VerifyPasskeyLoginAsync()
	{
		throw new NotImplementedException();
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

		var hash = Rfc2898DeriveBytes.Pbkdf2(password, salt, 10000, HashAlgorithmName.SHA1, 20);
		/* Compare the results */
		for (int i = 0; i < 20; i++)
			if (hashBytes[i + 16] != hash[i])
				return false;
		return true;
	}

	public async Task<List<PublicKeyCredentialDescriptor>> GetPublicKeyCredentialDescriptorsAsync(ObjectId id, CancellationToken cancellationToken = default)
	{
		var creds = await _users.Find(u => u.Id == id).Project(u => u.Credentials).FirstOrDefaultAsync(cancellationToken);
		return creds?.Select(c => c.Descriptor).ToList() ?? [];
	}

	public Task StoreCredentialsAsync(string credentialName, RegisteredPublicKeyCredential credential, CancellationToken cancellationToken = default)
	{
		var update = Builders<User>.Update
			.Push(u => u.Credentials, new StoredCredential(credentialName, credential));
		var userId = new ObjectId(credential.User.Id);
		return _users.UpdateOneAsync(u => u.Id == userId, update, null, cancellationToken);
	}

	public Task<bool> CredentialExistsAsync(byte[] credentialId, CancellationToken cancellationToken = default)
	{
		return _users.AsQueryable().AnyAsync(u => u.Credentials.Any(c => c.Descriptor.Id == credentialId), cancellationToken);
	}

	public async Task<StoredCredential?> GetStoredCredentialAsync(byte[] credentialId, CancellationToken cancellationToken = default)
	{
		var creds = await _users.Find(u => u.Credentials.Any(c => c.Descriptor.Id == credentialId))
			.Project(u => u.Credentials)
			.FirstOrDefaultAsync(cancellationToken);
		return creds?.FirstOrDefault(c => c.Descriptor.Id.Zip(credentialId).All((e) => e.First == e.Second));
	}

	public Task<bool> UserOwnsCredentialAsync(ObjectId userId, byte[] credentialId, CancellationToken cancellationToken = default)
	{
		return _users.AsQueryable().AnyAsync(u => u.Id == userId && u.Credentials.Any(c => c.Descriptor.Id == credentialId), cancellationToken);
	}

	public async Task<User?> VerifyPasskeyLoginAsync(ObjectId userId, VerifyAssertionResult result, CancellationToken cancellationToken = default)
	{
		var user = await _users.Find(u => u.Id == userId).FirstOrDefaultAsync();
		if (user == null)
			return null;

		var cred = user.Credentials.FirstOrDefault(c => c.Descriptor.Id.Zip(result.CredentialId).All(e => e.First == e.Second));
		if(cred == null)
			return null;

		cred.Counter = result.SignCount;
		cred.LastUsed = DateTimeOffset.Now;

		var update = Builders<User>.Update.Set(u => u.Credentials, user.Credentials);
		await _users.UpdateOneAsync(u => u.Id == userId, update, null, cancellationToken);

		return user;
	}
	
}
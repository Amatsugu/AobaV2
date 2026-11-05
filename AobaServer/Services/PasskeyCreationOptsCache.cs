using Fido2NetLib;


using MongoDB.Bson;

using System.Runtime.Caching;
using System.Diagnostics.CodeAnalysis;

namespace AobaServer.Services;

public class OptsCache<T> where T: class
{
	public MemoryCache Cache { get; } = new MemoryCache(nameof(T));

	public bool Add(ObjectId key, T value)
	{
		var cacheKey = key.ToString();
		return Cache.Add(cacheKey, value, new CacheItemPolicy
		{
			AbsoluteExpiration = DateTime.Now.AddSeconds(5),
		});
	}

	public bool TryRemove(ObjectId key, [NotNullWhen(true)] out T? value)
	{
		value = Cache.Remove(key.ToString()) as T;
		return value is not null;
	}
}

public class PasskeyCreationOptsCache : OptsCache<CredentialCreateOptions>;

public class PasskeyAssertionOptsCache : OptsCache<AssertionOptions>;

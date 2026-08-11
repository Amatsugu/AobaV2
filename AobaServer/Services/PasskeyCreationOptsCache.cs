using Fido2NetLib;

using MongoDB.Bson;

using System.Collections.Concurrent;

namespace AobaServer.Services;

public class PasskeyCreationOptsCache : ConcurrentDictionary<ObjectId, CredentialCreateOptions>
{
}

public class PasskeyAssertionOptsCache : ConcurrentDictionary<ObjectId, AssertionOptions>
{ }
using Fido2NetLib.Objects;

using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;

using System;
using System.Collections.Generic;
using System.Text;

namespace AobaCore.Models;

public class StoredCredential(string displayName, RegisteredPublicKeyCredential credential)
{
	public string DisplayName { get; set; } = displayName;
	public byte[] PublicKey { get; set; } = credential.PublicKey;
	public uint Counter { get; set; } = credential.SignCount;
	public PublicKeyCredentialDescriptor Descriptor { get; set; } = new PublicKeyCredentialDescriptor(credential.Id);
	public string CredType { get; set; } = credential.Type.ToString();
	public DateTimeOffset CreationTime { get; set; } = DateTimeOffset.UtcNow;
	public DateTimeOffset LastUsed { get; set; }
}
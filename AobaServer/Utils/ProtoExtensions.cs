using AobaCore.Models;
using Aoba.RPC;

using MongoDB.Bson;
using Google.Protobuf.WellKnownTypes;
using AobaCore.Services;
using Google.Protobuf.Collections;
using System.Net.Mail;
using Aoba.RPC.Models;
using SharpCompress.Compressors.ZStandard.Unsafe;

namespace AobaServer.Utils;

public static class ProtoExtensions
{
	public static ListResponse ToResponse(this PagedResult<Media> result, HostInfo host)
	{
		var res = new ListResponse()
		{
			Pagination = result.ToPagination(),
		};
		res.Items.AddRange(result.Items.Select(i => i.ToMediaModel(host)));
		return res;
	}

	public static Pagination ToPagination<T>(this PagedResult<T> result)
	{
		var p = new Pagination()
		{
			Page = result.Page,
			PageSize = result.PageSize,
			TotalItems = result.TotalItems,
			TotalPages = result.TotalPages,
		};
		if (result.Query != null)
			p.Query = result.Query;
		return p;
	}

	public static MediaResponse ToResponse(this Media? media, HostInfo host)
	{
		if (media == null)
			return new MediaResponse() { };
		return new MediaResponse()
		{
			Value = media.ToMediaModel(host)
		};
	}

	public static MediaModel ToMediaModel(this Media media, HostInfo host)
	{
		var thumbs = System.Enum.GetValues<AobaCore.Models.ThumbnailSize>().Select(s => new Aoba.RPC.Models.MediaThumbnail
		{
			Size = (Aoba.RPC.Models.ThumbnailSize)s,
			Url = media.GetThumbnailUrl(s, host)
		});
		var result = new MediaModel()
		{
			Ext = media.Ext,
			Filename = media.Filename,
			Id = media.MediaId.ToRpc(),
			MediaType = (Aoba.RPC.Models.MediaType)(media.MediaType + 1),
			Owner = media.Owner.ToRpc(),
			ViewCount = media.ViewCount,
			MainThumbUrl = media.GetThumbnailUrl(AobaCore.Models.ThumbnailSize.ResponsiveMedium, host),
			MediaUrl = media.GetMediaUrl(host),
			CdnUrl = media.GetRawMediaUrl(host),
			Class = (Aoba.RPC.Models.MediaClass)(media.Class + 1),
			MainThumbPending = media.Cdn?.ThumbnailUrls.ContainsKey(AobaCore.Models.ThumbnailSize.ResponsiveMedium) == false
		};
		if (media.Metadata?.Dimensions is SixLabors.ImageSharp.Size s)
		{
			result.Dimensions = new Dimensions
			{
				Height = (uint)s.Height,
				Width = (uint)s.Width,
			};
		}
		var (w, h) = AobaCore.Models.ThumbnailSize.ResponsiveMedium.GetThumbnailSize(media.Metadata?.Dimensions);
		result.MainThumbSize = new Dimensions
		{
			Width = (uint)w,
			Height = (uint)h
		};
		result.Thumbnails.AddRange(thumbs);
		result.Tags.AddRange(media.Tags);
		return result;
	}

	public static Id ToRpc(this ObjectId id)
	{
		return new Id() { Value = id.ToString() };
	}

	public static ObjectId ToObjectId(this Id id)
	{
		return id.Value.ToObjectId();
	}

	public static IEnumerable<ObjectId> ToObjectId(this IdList id)
	{
		return id.Value.Select(v => v.ToObjectId());
	}

	public static AobaCore.Models.MediaClass FromRPC(this Aoba.RPC.Models.MediaClass mediaClass) => (AobaCore.Models.MediaClass)(mediaClass - 1);


	public static PasskeyCreationResponse ToResponse(this Fido2NetLib.CredentialCreateOptions opts)
	{
		var result = new PasskeyCreationResponse
		{
			Options = new PasskeyCredentialCreateOptions
			{
				Challenge = Google.Protobuf.ByteString.CopyFrom(opts.Challenge),
				User = new PublicKeyCredentialUser
				{
					DisplayName = opts.User.DisplayName,
					Name = opts.User.Name,
					Id = Google.Protobuf.ByteString.CopyFrom(opts.User.Id)
				},
				Rp = new PublicKeyCredentialRpEntity
				{
					Id = opts.Rp.Id,
					Name = opts.Rp.Name
				},
			}
		};
		if (opts.Rp.Icon != null)
			result.Options.Rp.Icon = opts.Rp.Icon;
		result.Options.PubkeyParams.AddRange(opts.PubKeyCredParams.Select(cred => new PubKeyCredParam
		{
			Alg = (int)cred.Alg,
			Type = cred.Type switch
			{
				Fido2NetLib.Objects.PublicKeyCredentialType.PublicKey => "public-key",
				Fido2NetLib.Objects.PublicKeyCredentialType.Invalid => "invalid",
				_ => throw new InvalidOperationException($"Unknown cred type: {cred.Type}")
			}
		}));

		return result;
	}

	public static PasskeyAssertionResponse ToResponse(this Fido2NetLib.AssertionOptions options, ObjectId ceremonyId)
	{
		var res = new PasskeyAssertionResponse
		{
			Options = new PasskeyAssertionOptions
			{
				CeremonyId = ceremonyId.ToRpc(),
				Challenge = Google.Protobuf.ByteString.CopyFrom(options.Challenge),
			}
		};

		if (options.RpId != null)
			res.Options.RpId = options.RpId;

		return res;
	}

	public static Aoba.RPC.Models.TagGroup ToRpc(this AobaCore.Models.TagGroup tag)
	{
		return new Aoba.RPC.Models.TagGroup
		{
			Count = tag.Count,
			Tag = tag.Tag
		};
	}

	public static BulkTagInfo ToResponse(this List<AobaCore.Models.TagGroup> tags)
	{
		var result = new BulkTagInfo();
		result.Tags.AddRange(tags.Select(t => t.ToRpc()));
		return result;
	}

	public static Aoba.RPC.Models.UserSession ToRpc(this AobaCore.Models.UserSession session, ObjectId currentTokenId)
	{
		return new()
		{
			DateCreated = session.DateCreated.ToString("u"),
			LastUsed = session.LastUsed.ToString("u"),
			Label = session.Label,
			TokenId = session.TokenId.ToRpc(),
			UserAgent = session.UserAgent,
			IsCurrent = session.TokenId == currentTokenId
		};
	}

	public static IEnumerable<Aoba.RPC.Models.UserSession> ToRpc(this IEnumerable<AobaCore.Models.UserSession> sessions, ObjectId currentTokenId)
	{
		return sessions.Select(s => s.ToRpc(currentTokenId));
	}

	public static Aoba.RPC.Models.AccessToken ToRpc(this AobaCore.Models.AccessToken token)
	{
		return new Aoba.RPC.Models.AccessToken
		{
			DateCreated = token.IssuedDate.ToString("u"),
			Label = token.Label,
			LastUsed = token.LastUsed.ToString("u"),
			TokenId = token.TokenId.ToRpc()
		};
	}

	public static IEnumerable<Aoba.RPC.Models.AccessToken> ToRpc(this IEnumerable<AobaCore.Models.AccessToken> tokens)
	{
		return tokens.Select(t => t.ToRpc());
	}

	public static AobaCore.Models.ThumbnailSize FromRpc(this Aoba.RPC.Models.ThumbnailSize size)
	{
		return size switch
		{
			Aoba.RPC.Models.ThumbnailSize.Small => AobaCore.Models.ThumbnailSize.Small,
			Aoba.RPC.Models.ThumbnailSize.Medium => AobaCore.Models.ThumbnailSize.Medium,
			Aoba.RPC.Models.ThumbnailSize.Large => AobaCore.Models.ThumbnailSize.Large,
			Aoba.RPC.Models.ThumbnailSize.ExtraLarge => AobaCore.Models.ThumbnailSize.ExtraLarge,
			Aoba.RPC.Models.ThumbnailSize.ResponsiveSmall => AobaCore.Models.ThumbnailSize.ResponsiveSmall,
			Aoba.RPC.Models.ThumbnailSize.ResponsiveMedium => AobaCore.Models.ThumbnailSize.ResponsiveMedium,
			Aoba.RPC.Models.ThumbnailSize.ResponsiveLarge => AobaCore.Models.ThumbnailSize.ResponsiveLarge,
			_ => throw new InvalidOperationException("Invalid Thumbnail Size"),
		};
	}
}

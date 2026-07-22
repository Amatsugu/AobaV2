using AobaCore.Models;
using Aoba.RPC;

using MongoDB.Bson;
using Google.Protobuf.WellKnownTypes;
using AobaCore.Services;

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
		var p =new Pagination()
		{
			Page = result.Page,
			PageSize = result.PageSize,
			TotalItems = result.TotalItems,
			TotalPages = result.TotalPages,
		};
		if(result.Query != null)
			p.Query = result.Query;
		return p;
	}

	public static MediaResponse ToResponse(this Media? media, HostInfo host)
	{
		if(media == null)
			return new MediaResponse() {};
		return new MediaResponse()
		{
			Value = media.ToMediaModel(host)
		};
	}

	public static MediaModel ToMediaModel(this Media media, HostInfo host)
	{

		return new MediaModel()
		{
			Ext = media.Ext,
			Filename = media.Filename,
			Id = media.MediaId.ToId(),
			MediaType = (Aoba.RPC.MediaType)(media.MediaType + 1),
			Owner = media.Owner.ToId(),
			ViewCount = media.ViewCount,
			ThumbUrl = media.GetThumbnailUrl(ThumbnailSize.Medium, host),
			MediaUrl = media.GetMediaUrl(host),
			Class = (Aoba.RPC.MediaClass)(media.Class + 1),
		};
	}

	public static Id ToId(this ObjectId id)
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

	public static AobaCore.Models.MediaClass FromRPC(this Aoba.RPC.MediaClass mediaClass) => (AobaCore.Models.MediaClass)(mediaClass - 1);
}

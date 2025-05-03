using AobaCore.Models;
using Aoba.RPC;

using MongoDB.Bson;

namespace AobaServer.Utils;

public static class ProtoExtensions
{
	public static ListResponse ToResponse(this PagedResult<Media> result)
	{
		var res = new ListResponse()
		{
			Pagination = result.ToPagination(),
		};
		res.Items.AddRange(result.Items.Select(i => i.ToMediaModel()));
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

	public static MediaResponse ToResponse(this Media? media)
	{
		if(media == null) 
			return new MediaResponse() { Empty = new Empty() };
		return new MediaResponse()
		{
			Value = media.ToMediaModel()
		};
	}

	public static MediaModel ToMediaModel(this Media media)
	{
		return new MediaModel()
		{
			Ext = media.Ext,
			FileName = media.Filename,
			Id = media.Id.ToId(),
			MediaId = media.MediaId.ToId(),
			MediaType = (Aoba.RPC.MediaType)media.MediaType,
			Owner = media.Owner.ToId(),
			ViewCount = media.ViewCount,
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
}

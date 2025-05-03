using MongoDB.Bson;

namespace AobaServer.Utils;

public static class Extensions
{
	public static ObjectId ToObjectId(this string? value)
	{
		if(value == null)
			return ObjectId.Empty;
		if(ObjectId.TryParse(value, out ObjectId result))
			return result;
		return ObjectId.Empty;
	}
}

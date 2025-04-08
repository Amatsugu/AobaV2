using Microsoft.AspNetCore.Mvc.ModelBinding;
using MongoDB.Bson;

namespace AobaV2.Models;

public class BsonIdModelBinderProvider : IModelBinderProvider
{
	public IModelBinder? GetBinder(ModelBinderProviderContext context)
	{
		if (context.Metadata.ModelType == typeof(ObjectId))
			return new BsonIdModelBinder();
		return default;
	}
}

public class BsonIdModelBinder : IModelBinder
{
	public Task BindModelAsync(ModelBindingContext bindingContext)
	{
		var value = bindingContext.ValueProvider.GetValue(bindingContext.ModelName);
		if (value == ValueProviderResult.None)
			return Task.CompletedTask;

		if (ObjectId.TryParse(value.FirstValue, out var id))
			bindingContext.Result = ModelBindingResult.Success(id);
		else
			bindingContext.Result = ModelBindingResult.Failed();

			return Task.CompletedTask;
	}
}

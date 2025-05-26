global using MaybeError;

using AobaCore.Services;

using Microsoft.Extensions.DependencyInjection;

using MongoDB.Driver;
using MongoDB.Driver.Core.Extensions.DiagnosticSources;

using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AobaCore;
public static class Extensions
{
	public static IServiceCollection AddAoba(this IServiceCollection services, string dbString)
	{
		var settings = MongoClientSettings.FromConnectionString(dbString);
		settings.ClusterConfigurator = cb => cb.Subscribe(new DiagnosticsActivityEventSubscriber());
		var dbClient = new MongoClient(settings);
		var db = dbClient.GetDatabase("Aoba");

		services.AddSingleton(dbClient);
		services.AddSingleton<IMongoDatabase>(db);
		services.AddSingleton<AobaService>();
        services.AddSingleton<ThumbnailService>();
		services.AddSingleton<AccountsService>();
		services.AddHostedService<AobaIndexCreationService>();
		return services;
	}

	public static async Task EnsureIndexAsync<T>(this IMongoCollection<T> collection, CreateIndexModel<T> indexModel)
	{
		try
		{
			await collection.Indexes.CreateOneAsync(indexModel);
		}
		catch (MongoCommandException e) when (e.Code == 85 || e.Code == 86) //CodeName	"IndexOptionsConflict" or "NameConflict"
		{
			await collection.Indexes.DropOneAsync(indexModel.Options.Name);
			await collection.Indexes.CreateOneAsync(indexModel);
		}
	}
}

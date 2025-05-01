global using MaybeError;
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
	public static IServiceCollection AddAoba(this IServiceCollection services)
	{
		var settings = MongoClientSettings.FromConnectionString("mongodb://NinoIna:27017");
		settings.ClusterConfigurator = cb => cb.Subscribe(new DiagnosticsActivityEventSubscriber());
		var dbClient = new MongoClient(settings);
		var db = dbClient.GetDatabase("Aoba");

		services.AddSingleton(dbClient);
		services.AddSingleton<IMongoDatabase>(db);
		services.AddSingleton<AobaService>();
		return services;
	}
}

global using MaybeError;
using Microsoft.Extensions.DependencyInjection;

using MongoDB.Driver;

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
		var dbClient = new MongoClient("mongodb://NinoIna:27017");
		var db = dbClient.GetDatabase("Aoba");

		services.AddSingleton(dbClient);
		services.AddSingleton<IMongoDatabase>(db);
		services.AddSingleton<AobaService>();
		services.AddSingleton<MediaService>();
		return services;
	}
}

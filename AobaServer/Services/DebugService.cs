using AobaCore.Models;
using AobaCore.Services;

using MongoDB.Driver;

namespace AobaServer.Services;

public class DebugService(IMongoDatabase db, AutoTagger tagger) : BackgroundService
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>("media");

	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
	}
}

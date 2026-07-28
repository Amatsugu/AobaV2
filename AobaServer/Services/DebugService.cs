using AobaCore.Models;
using AobaCore.Services;

using MongoDB.Driver;

namespace AobaServer.Services;

public class DebugService(IMongoDatabase db, AobaService aobaService) : BackgroundService
{
	private IMongoCollection<Media> _media = db.GetCollection<Media>("media");
	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
	}
}

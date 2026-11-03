#if DEBUG
using System.Text.Json;
using AobaCore.Models;
using AobaCore.Services;

using MongoDB.Driver;

namespace AobaServer.Services;

public class DebugService(IMongoDatabase db, AccountsService accountsService, AobaService aobaService, ILogger<DebugService> logger) : BackgroundService
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>("media");
	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
		if (await accountsService.CreateDevAccountAsync())
			logger.LogInformation("Dev Account Created! User: dev, Password: dev");
		var ids = await _media.Find("{}").Limit(50).Project(m => m.MediaId).ToListAsync(stoppingToken);
		var info = await aobaService.GetBulkTagInfoAsync(ids);
		Console.WriteLine($"info:\n{JsonSerializer.Serialize(info)}");
	}
}

#endif
#if DEBUG
using AobaCore.Models;
using AobaCore.Services;

using MongoDB.Driver;

namespace AobaServer.Services;

public class DebugService(IMongoDatabase db, AccountsService accountsService, ThumbnailService thumbnails, ILogger<DebugService> logger) : BackgroundService
{
	private readonly IMongoCollection<Media> _media = db.GetCollection<Media>("media");
	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
		await accountsService.CreateDevAccountAsync();
	}
}

#endif
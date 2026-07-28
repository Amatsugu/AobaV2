using AobaCore.Models;
using AobaCore.Services;

using MongoDB.Driver;

namespace AobaServer.Services;

public class DebugService() : BackgroundService
{
	protected override async Task ExecuteAsync(CancellationToken stoppingToken)
	{
	}
}

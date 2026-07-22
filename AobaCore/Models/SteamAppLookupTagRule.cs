using Flurl;
using Flurl.Http;

using Microsoft.Extensions.Configuration;

using System;
using System.Collections.Generic;
using System.Text;
using System.Text.RegularExpressions;

namespace AobaCore.Models;

public partial class SteamAppLookupTagRule(IConfiguration config) : ITagRule
{
	public string? SteamApiKey => config["STEAM_API_KEY"];

	public async Task<Maybe<string[]>> GetTagsAsync(string filename)
	{
		try
		{
			var appId = SteamAppId().Match(filename).Groups.Values.FirstOrDefault(g => g.Name == "id")?.Value;
			if (appId == null)
				return Array.Empty<string>();
			var name = await GetGameNameAsync(appId);
			if (name == null)
				return Array.Empty<string>();
			return new string[] { name };
		}
		catch (Exception ex)
		{
			return ex;
		}
	}

	private async Task<string?> GetGameNameAsync(string appId)
	{
		throw new NotImplementedException();
	}

	private async Task LoadAppIdsAsync(string? lastAppId = null)
	{
		if (SteamApiKey == null)
			return;
		var result = await "https://partner.steam-api.com/IStoreService/GetAppList/v1/"
			.SetQueryParam("key", SteamApiKey)
			.SetQueryParam("last_appid", lastAppId)
			.GetJsonAsync<object>();
	}

	[GeneratedRegex("(?'id'\\d+)_(?:\\d+)_.*")]
	public static partial Regex SteamAppId();
}
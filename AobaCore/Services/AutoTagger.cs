using AobaCore.Models;

using System;
using System.Collections.Generic;
using System.Text;

namespace AobaCore.Services;

public class AutoTagger
{
	private static readonly List<ITagRule> _rules = [
		new SteamAppIdTagRule("220200", ["Kerbal Space Program", "KSP", "game"]),
		new SteamAppIdTagRule("3489700", ["Stellar Blade", "game"]),
		new SteamAppIdTagRule("3280350", ["DS2", "Death Stranding 2", "game"]),
		new SteamAppIdTagRule("1366540", ["Dyson Sphere Program", "game"]),
		new SteamAppIdTagRule("105600", ["Terraria", "game"]),
		new SteamAppIdTagRule("2989270", ["A Tithe in Blood", "game"]),
		new SteamAppIdTagRule("2246340", ["Monster Hunter Wilds", "game"]),
		new SteamAppIdTagRule("15510389888267583488", ["Cyberpunk 2077", "game"]),
		new SteamAppIdTagRule("275850", ["No Man's Sky", "game"]),
		new SteamAppIdTagRule("1196590", ["Resident Evil Village", "game"]),
		new SteamAppIdTagRule("1458140", ["Pacific Drive", "game"]),
		new SteamAppIdTagRule("427520", ["Factorio", "game"]),
		new SteamAppIdTagRule("2694490", ["Path of Exile 2", "POE2", "game"]),
		new SteamAppIdTagRule("582010", ["Monster Hunter", "game"]),
		new SteamAppIdTagRule("1282100", ["REMNANT II", "game"]),
		new SteamAppIdTagRule("412830", ["STEINS;GATE", "STEINS GATE", "game"]),
		new SteamAppIdTagRule("2420110", ["Horizon Forbidden West™ Complete Edition", "game"]),
		new SteamAppIdTagRule("294100", ["Rimworld", "game"]),
		new SteamAppIdTagRule("230410", ["Warframe", "game"]),
		new SteamAppIdTagRule("8930", ["Sid Meier's Civilization® V", "game"]),
		];

	public async Task<IEnumerable<string>> GetTagsAsync(string filename)
	{
		var results = new List<string>();
		foreach (var rule in _rules)
		{
			var tags = await rule.GetTagsAsync(filename);
			if (tags.HasError)
				continue;
			results.AddRange(tags);
		}
		return results.Distinct();
	}
}
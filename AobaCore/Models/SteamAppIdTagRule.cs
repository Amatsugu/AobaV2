using System;
using System.Collections.Generic;
using System.Text;
using System.Text.RegularExpressions;

namespace AobaCore.Models;

internal record SteamAppIdTagRule(string AppId, string[] Tags) : RegexTagRule(new Regex($"({AppId})_(\\d+)_.*"), Tags);

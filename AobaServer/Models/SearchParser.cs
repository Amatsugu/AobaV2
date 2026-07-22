using AobaCore.Models;

using MongoDB.Driver;

using System.Text.RegularExpressions;

namespace AobaServer.Models;

public record FieldToken(string Key, List<string> Values)
{
	public string Value => Values.First();
};

public class SearchQuery
{
	public required string OriginalQuery { get; set; }
	public required string TextQuery { get; set; }
	public required List<FieldToken> Fields { get; set; }

	public static implicit operator SearchQuery(string value) => QueryParser.Parse(value);
	public static implicit operator FilterDefinition<Media>(SearchQuery query) => query.ToFilter();

	public override string ToString()
	{
		return OriginalQuery;
	}

	public FilterDefinition<Media> ToFilter()
	{
		var filters = new List<FilterDefinition<Media>>();
		foreach (var field in Fields)
		{
			switch (field.Key.ToLower())
			{
				case "class":
					var items = field.Values.Select<string, MediaClass?>(v => Enum.TryParse<MediaClass>(v, out var mClass) ? mClass : null)
						.Where(m => m != null)
						.Cast<MediaClass>();
					filters.Add(Builders<Media>.Filter.In(m => m.Class, items));
					break;
				case "tags":
					filters.Add(Builders<Media>.Filter.AnyIn(m => m.Tags, field.Values));
					break;
			}
		}
		if (!string.IsNullOrWhiteSpace(TextQuery))
			filters.Add(Builders<Media>.Filter.Text(TextQuery));
		else if(filters.Count == 0)
			filters.Add("{}");
		return Builders<Media>.Filter.And(filters);
	}
}

public static partial class QueryParser
{
	[GeneratedRegex(
		@"(?:(?<key>""[^""]+""|\w+):(?<value>(?:""[^""]+""|[^\s,]+)(?:,\s*(?:""[^""]+""|[^\s,]+))*))|(?<plain>""[^""]+""|\S+)")]
	private static partial Regex TokenRegex();

	[GeneratedRegex(@"""[^""]+""|[^\s,]+")]
	private static partial Regex ValueSplitRegex();

	private static string Unquote(string s)
	{
		if (s.Length >= 2 && s.StartsWith("\"") && s.EndsWith("\""))
			return s.Substring(1, s.Length - 2);
		return s;
	}

	public static SearchQuery Parse(string query)
	{
		var fields = new List<FieldToken>();
		var plainParts = new List<string>();

		foreach (Match m in TokenRegex().Matches(query))
		{
			if (m.Groups["key"].Success)
			{
				var values = new List<string>();
				foreach (Match v in ValueSplitRegex().Matches(m.Groups["value"].Value))
					values.Add(Unquote(v.Value));

				fields.Add(new FieldToken(Unquote(m.Groups["key"].Value), values));
			}
			else if (m.Groups["plain"].Success)
			{
				plainParts.Add(m.Groups["plain"].Value);
			}
		}

		return new SearchQuery
		{
			OriginalQuery = query,
			TextQuery = string.Join(" ", plainParts),
			Fields = fields
		};
	}
}
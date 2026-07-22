using System;
using System.Collections.Generic;
using System.Text;
using System.Text.RegularExpressions;

namespace AobaCore.Models;

public record RegexTagRule(Regex Pattern, string[] Tags) : ITagRule
{
	public Task<Maybe<string[]>> GetTagsAsync(string filename)
	{
		try
		{
			if (Pattern.IsMatch(filename))
				return Task.FromResult<Maybe<string[]>>(Tags);
			return Task.FromResult<Maybe<string[]>>(Array.Empty<string>());
		}
		catch (Exception ex)
		{
			return Task.FromResult<Maybe<string[]>>(ex);
		}
	}
}
namespace AobaCore.Models;

public interface ITagRule
{
	Task<Maybe<string[]>> GetTagsAsync(string filename);
}

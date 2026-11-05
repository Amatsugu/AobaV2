namespace AobaCore.Models;

public record TagGroup
{
	public string Tag { get; set; } = "";
	public int Count { get; set; }
};
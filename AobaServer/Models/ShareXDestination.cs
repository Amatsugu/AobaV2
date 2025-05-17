namespace AobaServer.Models;

public class ShareXDestination
{
	public string Version { get; set; } = "13.1.0";
	public string Name { get; set; } = "Aoba";
	public string DestinationType { get; set; } = "ImageUploader, TextUploader, FileUploader";
	public string RequestMethod { get; set; } = "POST";
	public string RequestURL { get; set; } = "https://aoba.app/api/media/upload";
	public Dictionary<string, string> Headers { get; set; } = [];
	public string Body { get; set; } = "MultipartFormData";
	public Dictionary<string, string> Arguments { get; set; } = new() { { "name", "$filename$" } };
	public string FileFormName { get; set; } = "file";
	public string[] RegexList { get; set; } = ["([^/]+)/?$"];
	public string URL { get; set; } = "https://aoba.app$json:url$";
	public required string ThumbnailURL { get; set; }
	public required string DeletionURL { get; set; }
}
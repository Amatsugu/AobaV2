using Microsoft.Extensions.Configuration;

namespace AobaCore.Services;

public class HostInfo(IConfiguration configuration)
{
	public string Host => configuration["HOST"] ?? throw new NullReferenceException("HOST is not set");
	public string CdnHost => configuration["CDN_HOST"] ?? Host;
}

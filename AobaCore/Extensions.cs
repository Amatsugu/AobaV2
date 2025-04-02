using Microsoft.Extensions.DependencyInjection;

using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AobaCore;
public static class Extensions
{
	public static IServiceCollection AddAoba(this IServiceCollection services)
	{
		services.AddSingleton<AobaService>();
		return services;
	}
}

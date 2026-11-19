namespace AobaServer.Middleware;

public static class Owasp
{
	/// <summary>
	/// Configures response headers based on best practices from https://secure-headers-project.owasp.org/index/
	/// </summary>
	/// <remarks>HSTS is handled on reverse proxy</remarks>
	/// <param name="applicationBuilder"></param>
	/// <returns></returns>
	public static IApplicationBuilder UseOwaspHeaders(this IApplicationBuilder applicationBuilder)
	{
		applicationBuilder.Use(async (ctx, next) =>
		{
			ctx.Response.OnStarting(() =>
			{
				var headers = ctx.Response.Headers;
				headers.CacheControl = "no-store, max-age=0";
				headers["X-Permitted-Cross-Domain-Policies"] = "none";
				headers["Referrer-Policy"] = "no-referrer";
				headers["Cross-Origin-Embedder-Policy"] = "require-corp";
				headers["Cross-Origin-Opener-Policy"] = "same-origin";
				headers["Cross-Origin-Resource-Policy"] = "same-origin";
				headers["Permissions-Policy"] = "accelerometer=(), autoplay=(), camera=(), cross-origin-isolated=(), display-capture=(), encrypted-media=(), fullscreen=(), geolocation=(), gyroscope=(), keyboard-map=(), magnetometer=(), microphone=(), midi=(), payment=(), picture-in-picture=(), screen-wake-lock=(), sync-xhr=(self), usb=(), web-share=(), xr-spatial-tracking=(), clipboard-read=(), clipboard-write=(), gamepad=(), hid=(), idle-detection=(), interest-cohort=(), serial=(), unload=()";
				headers["X-DNS-Prefetch-Control"] = "off";
				headers["X-XSS-Protection"] = "0";
				return Task.CompletedTask;
			});
			await next();
		});
		return applicationBuilder;
	}
}

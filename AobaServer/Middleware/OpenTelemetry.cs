#nullable enable

using AobaServer.Middleware;

using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Routing;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;

using OpenTelemetry;
using OpenTelemetry.Metrics;
using OpenTelemetry.Resources;
using OpenTelemetry.Trace;


namespace AobaServer.Middleware;

public static class OpenTelemetry
{
	public static void AddObersability(this IServiceCollection services, IConfiguration configuration)
	{
		var otel = services.AddOpenTelemetry();

		otel.ConfigureResource(res =>
		{
			res.AddService(serviceName: $"Breeze: {Environment.GetEnvironmentVariable("ASPNETCORE_ENVIRONMENT")}");
		});


		// Add Metrics for ASP.NET Core and our custom metrics and export to Prometheus
		otel.WithMetrics(metrics => metrics
			// Metrics provider from OpenTelemetry
			.AddAspNetCoreInstrumentation()
			.AddCustomMetrics()
			// Metrics provides by ASP.NET Core in .NET 8
			.AddMeter("Microsoft.AspNetCore.Hosting")
			.AddMeter("Microsoft.AspNetCore.Server.Kestrel")
			// Metrics provided by System.Net libraries
			.AddMeter("System.Net.Http")
			.AddMeter("System.Net.NameResolution")
			.AddMeter("MongoDB.Driver.Core.Extensions.DiagnosticSources")
			.AddPrometheusExporter());

		// Add Tracing for ASP.NET Core and our custom ActivitySource and export to Jaeger
		var tracingOtlpEndpoint = configuration["OTLP_ENDPOINT_URL"];
		otel.WithTracing(tracing =>
		{
			tracing.AddAspNetCoreInstrumentation();
			tracing.AddHttpClientInstrumentation();
			if (!string.IsNullOrWhiteSpace(tracingOtlpEndpoint))
			{
				tracing.AddOtlpExporter(otlpOptions =>
				{
					otlpOptions.Endpoint = new Uri(tracingOtlpEndpoint);
				});
			}
		});
	}


	public static MeterProviderBuilder AddCustomMetrics(this MeterProviderBuilder builder)
	{


		return builder;
	}

	public static IEndpointRouteBuilder MapObserability(this IEndpointRouteBuilder endpoints)
	{
		endpoints.MapPrometheusScrapingEndpoint().RequireAuthorization(p => p.RequireRole("metrics"));
		return endpoints;
	}
}

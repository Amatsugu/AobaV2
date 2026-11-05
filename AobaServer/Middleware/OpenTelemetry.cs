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
	public static void AddObservability(this IServiceCollection services, IConfiguration configuration)
	{
		var otel = services.AddOpenTelemetry();

		otel.ConfigureResource(res =>
		{
			res.AddService(serviceName: $"Aoba: {Environment.GetEnvironmentVariable("ASPNETCORE_ENVIRONMENT")}");
		});


		// Add Metrics for ASP.NET Core and our custom metrics and export to Prometheus
		otel.WithMetrics(metrics => metrics
			// Metrics provider from OpenTelemetry
			.AddAspNetCoreInstrumentation()
			.AddCustomMetrics()
			// Metrics provides by ASP.NET Core in .NET 8
			.AddMeter("Microsoft.AspNetCore.Hosting")
			.AddMeter("Microsoft.AspNetCore.Server.Kestrel")
			.AddMeter("Grpc.AspNetCore.Server")
			// Metrics provided by System.Net libraries
			.AddMeter("System.Net.Http")
			.AddMeter("System.Net.NameResolution")
			.AddMeter("System.Runtime")
			.AddMeter("Microsoft.AspNetCore.Diagnostics")
			.AddAspNetCoreInstrumentation()
			.AddHttpClientInstrumentation()
			.AddPrometheusExporter());

		var tracingOtlpEndpoint = configuration["OTLP_ENDPOINT_URL"];
		if (!string.IsNullOrWhiteSpace(tracingOtlpEndpoint))
		{
			otel.WithTracing(tracing =>
			{
				tracing.AddSource("MongoDB.Driver.Core.Extensions.DiagnosticSources");
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
	}


	public static MeterProviderBuilder AddCustomMetrics(this MeterProviderBuilder builder)
	{


		return builder;
	}

	public static IEndpointRouteBuilder MapObservability(this IEndpointRouteBuilder endpoints)
	{
		endpoints.MapPrometheusScrapingEndpoint().RequireAuthorization(Consts.MetricsAuthPolicy);
		return endpoints;
	}
}

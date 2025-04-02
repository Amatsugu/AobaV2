using Microsoft.AspNetCore.Http.Features;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
#if DEBUG
	builder.Services.AddControllersWithViews().AddRazorRuntimeCompilation();
	builder.Services.AddSassCompiler();
#else
	builder.Services.AddControllersWithViews();
#endif

builder.Services.Configure<FormOptions>(opt =>
{
	opt.ValueLengthLimit = int.MaxValue;
	opt.MultipartBodyLengthLimit = int.MaxValue;
});

var app = builder.Build();

// Configure the HTTP request pipeline.
if (!app.Environment.IsDevelopment())
{
    app.UseExceptionHandler("/Home/Error");
    // The default HSTS value is 30 days. You may want to change this for production scenarios, see https://aka.ms/aspnetcore-hsts.
    app.UseHsts();
}

//Javascript frameworks were a mistake
app.Use((c, n) =>
{
	if(!c.Request.Path.HasValue)
		return n.Invoke();
	if (c.Request.Path.Value.EndsWith(".js"))
		return n.Invoke();
	if (!(c.Request.Path.StartsWithSegments("/js") || c.Request.Path.StartsWithSegments("/lib")))
		return n.Invoke();

	c.Response.Redirect($"{c.Request.Path}.js{c.Request.QueryString}");
	c.Response.StatusCode = StatusCodes.Status301MovedPermanently;
	return Task.CompletedTask;
});

app.UseHttpsRedirection();
app.UseRouting();

app.UseAuthorization();

app.MapStaticAssets();

app.MapControllerRoute(
    name: "default",
    pattern: "{controller=Home}/{action=Index}/{id?}")
    .WithStaticAssets();


app.Run();

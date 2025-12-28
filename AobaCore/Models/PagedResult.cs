using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AobaCore.Models;
public class PagedResult<T>(List<T> items, int page, int pageSize, int totalItems)
{
	public List<T> Items { get; set; } = items;
	public int Page {  get; set; } = page;
	public int PageSize { get; set; } = pageSize;
	public int TotalItems { get; set; } = totalItems;
	public int TotalPages { get; set; } = (totalItems / pageSize) + 1;
	public string? Query { get; set; }
}



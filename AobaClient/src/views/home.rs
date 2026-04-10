use crate::{
	components::{MediaGrid, Pagination, PaginationInfo, Search},
	route::Route,
};
use dioxus::{prelude::*, router::RouterConfig};

// #[component]
// pub fn Home() -> Element
// {
// 	let query = use_signal(|| "".to_string());
// 	let page = use_signal(|| 1 as i32);
// 	let max_page = use_signal(|| 1 as i32);
// 	let item_count = use_signal(|| 0 as i32);
// 	rsx! {
// 		div	{
// 			class: "stickyTop",
// 			Search { query, page },
// 			Pagination { page, max_page, item_count },
// 		}
// 		MediaGrid { query: query.cloned(), page: page.cloned(), max_page, total_items: item_count }
// 	}
// }

#[component]
pub fn Home(page: Option<i32>, q: Option<String>) -> Element
{
	let mut query = use_signal(|| q.unwrap_or("".to_string()));
	let mut page = use_signal(|| page.unwrap_or(1));
	let page_size = use_signal::<i32>(|| 100);
	let mut max_page = use_signal(|| 1 as i32);
	let mut item_count = use_signal(|| 0 as i32);
	rsx! {
		div	{
			class: "stickyTop",
			Search {
				query: query(),
				oninput: move |q| {
					query.set(q);
					page.set(1);
				},
				onchange: move |_|{
					router().push(format!("/?page={}&q={}", page(), query()));
				}
			},
			Pagination {
				page, max_page, item_count,
				on_page_change: move |p|{
					page.set(p);
					router().push(format!("/?page={}&q={}", page(), query()));
				}
			},
		}
		MediaGrid { query: query, page: page, max_page, total_items: item_count, page_size,
			on_page_loaded: move |p: PaginationInfo| {
				max_page.set(p.total_pages);
				item_count.set(p.total_items);
			}
		}
	}
}

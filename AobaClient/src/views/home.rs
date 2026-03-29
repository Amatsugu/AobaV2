use crate::components::{MediaGrid, Pagination, Search};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element
{
	let query = use_signal(|| "".to_string());
	let page = use_signal(|| 1 as i32);
	let max_page = use_signal(|| 1 as i32);
	let item_count = use_signal(|| 0 as i32);
	rsx! {
		div	{
			class: "stickyTop",
			Search { query, page },
			Pagination { page, max_page, item_count },
		}
		MediaGrid { query: query.cloned(), page: page.cloned(), max_page, total_items: item_count }
	}
}

#[component]
pub fn HomePaged(page: i32, q: String) -> Element
{
	let query = use_signal(|| q);
	let page = use_signal(|| page);
	let max_page = use_signal(|| 1 as i32);
	let item_count = use_signal(|| 0 as i32);
	rsx! {
		div	{
			class: "stickyTop",
			Search { query, page },
			Pagination { page, max_page, item_count },
		}
		MediaGrid { query: query.cloned(), page: page.cloned(), max_page, total_items: item_count }
	}
}

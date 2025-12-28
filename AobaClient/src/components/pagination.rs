use dioxus::prelude::*;

#[component]
pub fn Pagination(page: Signal<i32>, max_page: Signal<i32>, item_count: Signal<i32>) -> Element {
	let cur_page_val = page.cloned();
	let max_page_val = max_page.cloned();
	let item_count_val = item_count.cloned();
	rsx! {
		div {
			class: "pagination",
			a {
				onclick: move|_| page.set(1),
				"First"
			}
			a {
				onclick: move|_| page.set((cur_page_val - 1).max(1)),
				"Prev"
			}
			div { "Page {cur_page_val} of {max_page_val} ({item_count_val} Media Items)" }
			a {
				onclick: move|_| page.set((cur_page_val + 1).min(max_page_val)),
				"Next"
			}
			a {
				onclick: move|_| page.set(max_page_val),
				"Last"
			}
		}
	}
}

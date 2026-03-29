use dioxus::prelude::*;
use web_sys::window;

#[component]
pub fn Pagination(page: Signal<i32>, max_page: Signal<i32>, item_count: Signal<i32>) -> Element
{
	let cur_page_val = page.cloned();
	let max_page_val = max_page.cloned();
	let item_count_val = item_count.cloned();
	rsx! {
		div {
			class: "pagination",
			a {
				onclick: move|_| {
					page.set(1);
					on_page_change();
				},
				"First"
			}
			a {
				onclick: move|_| {
					let p = (cur_page_val - 1).max(1);
					page.set(p);
					on_page_change();
				},
				"Prev"
			}
			div { "Page {cur_page_val} of {max_page_val} ({item_count_val} Media Items)" }
			a {
				onclick: move|_| {
					let p = (cur_page_val + 1).min(max_page_val);
					page.set(p);
					on_page_change();
				},
				"Next"
			}
			a {
				onclick: move|_| {
					page.set(max_page_val);
					on_page_change();
				},
				"Last"
			}
		}
	}
}

fn on_page_change()
{
	let window = window().expect("Failed to get window");
	let document = window.document().expect("Failed to get document");
	document
		.query_selector("#content")
		.expect("Failed to find content")
		.expect("Failed to find content")
		.scroll_to_with_x_and_y(0.0, 0.0);
}

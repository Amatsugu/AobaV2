use dioxus::prelude::*;
use web_sys::window;

#[component]
pub fn Pagination(
	page: Signal<i32>,
	max_page: Signal<i32>,
	item_count: Signal<i32>,
	on_page_change: EventHandler<i32>,
) -> Element {
	let cur_page_val = page.cloned();
	let max_page_val = max_page.cloned();
	let item_count_val = item_count.cloned();
	rsx! {
		div {
			class: "pagination",
			a {
				onclick: move|_| {
					on_page_change.call(1);
					scroll_document();
				},
				"First"
			}
			a {
				onclick: move|_| {
					let p = (cur_page_val - 1).max(1);
					on_page_change.call(p);
					scroll_document();
				},
				"Prev"
			}
			div { "Page {cur_page_val} of {max_page_val} ({item_count_val} Media Items)" }
			a {
				onclick: move|_| {
					let p = (cur_page_val + 1).min(max_page_val);
					on_page_change.call(p);
					scroll_document();
				},
				"Next"
			}
			a {
				onclick: move|_| {
					on_page_change.call(max_page_val);
					scroll_document();
				},
				"Last"
			}
		}
	}
}

fn scroll_document() {
	let window = window().expect("Failed to get window");
	let document = window.document().expect("Failed to get document");
	document
		.query_selector("#content")
		.expect("Failed to find content")
		.expect("Failed to find content")
		.scroll_to_with_x_and_y(0.0, 0.0);
}

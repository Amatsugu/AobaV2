use crate::components::{MediaGrid, Search};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
	let mut query = use_signal(|| "".to_string());

	rsx! {
		div {
			id: "content",
			Search {
				query: query.cloned(),
				oninput: move |event:FormEvent| {
					query.set(event.value())
				}
			},
			MediaGrid { query: query.cloned() }
		}
	}
}

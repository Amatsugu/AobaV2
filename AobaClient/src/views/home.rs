use crate::components::{MediaGrid, Search};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
	let query = use_signal(|| "".to_string());

	rsx! {
		Search {
			query: query
		},
		MediaGrid { query: query.cloned() }
	}
}

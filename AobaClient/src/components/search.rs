use dioxus::prelude::*;

#[component]
pub fn Search() -> Element {
	rsx! {
		div{
			class: "searchBar",
			input {
				type: "search",
				placeholder: "Search Files"
			}
		}
	}
}

use dioxus::prelude::*;

#[component]
pub fn Search(query: Signal<String>) -> Element {
	rsx! {
		div{
			class: "searchBar",
			input {
				type: "search",
				placeholder: "Search Files",
				value: query,
				oninput: move |event| query.set(event.value())
			}
		}
	}
}

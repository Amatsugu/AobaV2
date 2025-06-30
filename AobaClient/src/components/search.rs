use dioxus::prelude::*;

#[component]
pub fn Search(query: Signal<String>) -> Element {
	rsx! {
		div { class: "searchBar stickyTop",
			input {
				r#type: "search",
				placeholder: "Search Files",
				value: query,
				oninput: move |event| query.set(event.value()),
			}
		}
	}
}

use dioxus::prelude::*;

#[component]
pub fn Search(query: Signal<String>, page: Signal<i32>) -> Element {
	rsx! {
		div { class: "searchBar stickyTop",
			input {
				r#type: "search",
				placeholder: "Search Files",
				value: query,
				oninput: move |event| {query.set(event.value()); page.set(1);},
			}
		}
	}
}

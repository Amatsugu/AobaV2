use dioxus::prelude::*;

#[component]
pub fn Search(query: String, oninput: Option<EventHandler<String>>) -> Element {
	rsx! {
		div { class: "searchBar",
			input {
				r#type: "search",
				placeholder: "Search Files",
				value: query,
				oninput: move |event| {
					if let Some(handler) = oninput {
						handler.call(event.value());
					}
				},
			}
		}
	}
}

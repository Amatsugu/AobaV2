use dioxus::prelude::*;

#[component]
pub fn Search(query: Option<String>, oninput: EventHandler<FormEvent>) -> Element {
	rsx! {
		div{
			class: "searchBar",
			input {
				type: "search",
				placeholder: "Search Files",
				value: query.unwrap_or("".into()),
				oninput: move |event| oninput.call(event)
			}
		}
	}
}

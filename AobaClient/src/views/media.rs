use dioxus::prelude::*;

#[component]
pub fn Media(id: String) -> Element {
	rsx! {
		{id}
	}
}

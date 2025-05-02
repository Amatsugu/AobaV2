use crate::components::{MediaGrid, Search};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
	rsx! {
		div { id: "content",
			"This is home"
			Search {  },
			MediaGrid { }
		}
	}
}

use crate::models::media::Media;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
	rsx! {
		div { id: "content",
			"This is home"
		}
	}
}
#[component]
fn MediaDisplay(media: Media) -> Element {
	rsx! {
		div { "{media.id}" }
	}
}

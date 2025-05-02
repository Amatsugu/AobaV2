use dioxus::prelude::*;

#[component]
pub fn MediaGrid() -> Element {
	rsx! {
		div{
			class: "mediaGrid",
			{(0..50).map(|_| rsx!{
				MediaItem {}
			})}
		}
	}
}

#[component]
pub fn MediaItem() -> Element {
	rsx! {
		div{
			class: "mediaItem",
			img{}
			div {
				class: "info",
				span{
					class: "name",
					"Filename"
				},
				div{
					class: "details",
					span{
						"Type"
					},
					span{
						"View Count"
					},
				}
			}
		}
	}
}

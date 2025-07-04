use dioxus::prelude::*;

use crate::{HOST, rpc::aoba::MediaModel};

#[derive(PartialEq, Clone, Props)]
pub struct MediaItemProps {
	pub item: Option<MediaModel>,
}

#[component]
pub fn MediaItem(props: MediaItemProps) -> Element {
	if let Some(item) = props.item {
		let mtype = item.media_type().as_str_name();
		let filename = item.file_name;
		let id = item.media_id.unwrap().value;

		let src = format!("{HOST}/m/thumb/{id}");
		return rsx! {
			a { class: "mediaItem", href: "{HOST}/m/{id}", target: "_blank",
				img { src }
				span { class: "info",
					span { class: "name", "{filename}" }
					span { class: "details",
						span { "{mtype}" }
						span { "{item.view_count}" }
					}
				}
			}
		};
	} else {
		return rsx! {
			div { class: "mediaItem placeholder",
				img { },
				span { class: "info",
					span { class: "name" }
					span { class: "details",
						span { }
						span { }
					}
				}
			}
		};
	}
}

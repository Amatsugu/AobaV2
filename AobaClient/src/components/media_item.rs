use dioxus::prelude::*;

use crate::{HOST, rpc::aoba::MediaModel};

#[derive(PartialEq, Clone, Props)]
pub struct MediaItemProps {
	pub item: MediaModel,
}

#[component]
pub fn MediaItem(props: MediaItemProps) -> Element {
	let mtype = props.item.media_type().as_str_name();
	let filename = props.item.file_name;
	let id = props.item.media_id.unwrap().value;

	let src = format!("{HOST}/m/thumb/{id}");
	rsx! {
		a { class: "mediaItem", href: "{HOST}/m/{id}", target: "_blank",
			img { src }
			span { class: "info",
				span { class: "name", "{filename}" }
				span { class: "details",
					span { "{mtype}" }
					span { "{props.item.view_count}" }
				}
			}
		}
	}
}

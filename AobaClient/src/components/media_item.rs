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

	#[cfg(debug_assertions)]
	let src = format!("{HOST}/m/thumb/{id}");
	#[cfg(not(debug_assertions))]
	let src = format!("https://aoba.app/m/thumb/{id}");
	// let url = "https://aoba.app/i/{}";
	rsx! {
		div{
			class: "mediaItem",
			img{ src: src }
			div {
				class: "info",
				span{
					class: "name",
					"{filename}"
				},
				div{
					class: "details",
					span{
						"{mtype}"
					},
					span{
						"{props.item.view_count}"
					},
				}
			}
		}
	}
}

use dioxus::prelude::*;

use crate::{HOST, rpc::aoba::MediaModel};

#[derive(PartialEq, Clone, Props)]
pub struct MediaItemProps {
	pub item: Option<MediaModel>,
	pub oncontextmenu: Option<EventHandler<Event<MouseData>>>,
}

#[component]
pub fn MediaItem(props: MediaItemProps) -> Element {
	if let Some(item) = props.item {
		let mtype = item.media_type().as_str_name();
		let filename = item.file_name;
		let id = item.id.unwrap().value;
		let thumb = item.thumb_url;
		let url = item.media_url;

		return rsx! {
			a {
				class: "mediaItem",
				href: "{HOST}/{url}",
				target: "_blank",
				oncontextmenu: move |e| {
					if let Some(handler) = props.oncontextmenu{
						handler.call(e);
					}
				},
				"data-id" : id,
				img { src: "{HOST}{thumb}" }
				span { class: "info",
					span { class: "name", "{filename}" }
					span { class: "details",
						span { "{mtype}" }
						span { "{item.view_count}" }
					}
				}
			},
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

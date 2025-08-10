use dioxus::prelude::*;
use web_sys::window;

use crate::{
	HOST,
	components::{ContextMenu, ContextMenuItem, ContextMenuRenderer},
	rpc::aoba::MediaModel,
};

#[derive(PartialEq, Clone, Props)]
pub struct MediaItemProps {
	pub item: Option<MediaModel>,
	// pub oncontextmenu: Option<EventHandler<Event<MouseData>>>,
}

#[component]
pub fn MediaItem(props: MediaItemProps) -> Element {
	let mut ct_renderer = use_context::<ContextMenuRenderer>();

	if let Some(item) = props.item {
		let mtype = item.media_type().as_str_name();
		let filename = item.file_name;
		let id = item.id.unwrap().value;
		let thumb = item.thumb_url;
		let url = item.media_url;
		let download = format!("{HOST}{url}");

		let oncontext = move |event: Event<MouseData>| {
			println!("ContextMenu");
			event.prevent_default();
			event.stop_propagation();
			let data = event.data();
			if data.modifiers().ctrl() {
				return;
			}
			let pos = data.coordinates().client();
			let left = pos.x;
			let top = pos.y;
			let download = download.clone();

			let menu: Element = rsx! {
				ContextMenu {
					left: left,
					top: top,
					items: rsx! {
						ContextMenuItem {
							name: "Details",
						},
						ContextMenuItem {
							name: "Download",
							onclick: move |_|{
								_ = window().unwrap().open_with_url_and_target(&download, "_blank");
							}
						},
						ContextMenuItem {
							name: "Delete",
						},
					},
				}
			};
			ct_renderer.menu.set(Some(menu));
		};

		return rsx! {
			a {
				class: "mediaItem",
				href: "{HOST}{url}",
				target: "_blank",
				oncontextmenu: oncontext,
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

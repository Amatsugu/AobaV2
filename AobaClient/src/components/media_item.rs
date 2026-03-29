use dioxus::prelude::*;
use dioxus_primitives::context_menu::{ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger};
use tonic::{Response, Status};
use web_sys::window;

use crate::{
	HOST,
	route::Route,
	rpc::{
		aoba::{Id, MediaClass, MediaModel, SetMediaClassRequest},
		get_rpc_client,
	},
};

#[derive(PartialEq, Clone, Props)]
pub struct MediaItemProps
{
	pub item: Option<MediaModel>,
	// pub oncontextmenu: Option<EventHandler<Event<MouseData>>>,
}

#[component]
pub fn MediaItem(props: MediaItemProps) -> Element
{
	let mut class_signal = use_signal(|| "");
	if let Some(item) = props.item
	{
		let mtype = item.media_type().as_str_name();
		let filename = item.file_name;
		let id = item.id.unwrap().value;
		let thumb = item.thumb_url;
		let class = item.class;
		let url = item.media_url;
		let download = format!("{HOST}{url}");

		match class
		{
			1 => class_signal.set("nsfw"),
			2 => class_signal.set("secret"),
			_ => class_signal.set(""),
		};

		return rsx! {
			ContextMenu{
				ContextMenuTrigger{
					a {
						class: "mediaItem {class_signal()}",
						href: "{HOST}{url}",
						target: "_blank",
						"data-id" : id.clone(),
						img { src: "{HOST}{thumb}" }
						span { class: "info",
							span { class: "name", "{filename}" }
							span { class: "details",
								span { "{mtype}" }
								span { "{item.view_count}" }
							}
						}
					},
				},
				ContextMenuContent{
					ContextMenuItem {
						index: 0 as usize,
						value: id.clone(),
						on_select: move |id: String|{
							window().expect("Failed to get window")
								.location().set_href(&format!("/media/{}", id))
								.expect("Failed to open Url");
						},
						div{
							class: "contextItem",
							div{
								class: "label",
								"Details"
							}
						}
					},
					ContextMenuItem {
						index: 1 as usize,
						value: "{download}",
						on_select: move |url: String|{
							window().expect("Failed to get window").open_with_url_and_target(&url, "_blank").expect("Failed to open url");
						},
						div{
							class: "contextItem",
							div{
								class: "label",
								"Download"
							}
						}
					},
					{
						if class != 0 {
							rsx!{ContextMenuItem {
								index: 2 as usize,
								value: "{id}",
								on_select: async |id: String|{
									_ = set_class(id, 0).await;
								},
								div{
									class: "contextItem",
									div{
										class: "label",
										"Mark Standard"
									}
								}
							}}
						}else{
							rsx!{}
						}
					}
					{
						if class != 1 {
							rsx!{ContextMenuItem {
								index: 2 as usize,
								value: "{id}",
								on_select: async |id: String|{
									_ = set_class(id, 1).await;
								},
								div{
									class: "contextItem",
									div{
										class: "label",
										"Mark NSFW"
									}
								}
							}}
						}else{
							rsx!{}
						}
					}
					{
						if class != 1 {
							rsx!{ContextMenuItem {
								index: 2 as usize,
								value: "{id}",
								on_select: async |id: String|{
									_ = set_class(id, 2).await;
								},
								div{
									class: "contextItem",
									div{
										class: "label",
										"Mark Secret"
									}
								}
							}}
						}else{
							rsx!{}
						}
					}
					ContextMenuItem {
						index: 2 as usize,
						value: "",
						div{
							class: "contextItem",
							div{
								class: "label",
								"Delete"
							}
						}
					},
				}
			}
		};
	}
	else
	{
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

async fn set_class(id: String, class: i32) -> Result<Response<()>, Status>
{
	let mut client = get_rpc_client();
	return client
		.set_media_class(SetMediaClassRequest {
			class: class,
			id: Some(Id { value: id }),
		})
		.await;
}

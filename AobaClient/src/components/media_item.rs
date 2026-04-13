use dioxus::prelude::*;
use dioxus_primitives::context_menu::{ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger};
use tonic::{Response, Status};
use web_sys::window;

use crate::{
	HOST,
	rpc::{
		aoba::{Id, MediaModel, SetMediaClassRequest},
		get_rpc_client,
	},
};

pub struct MediaClassChangeEvent
{
	pub id: String,
	pub class: i32,
}

#[derive(PartialEq, Clone, Props)]
pub struct MediaItemProps
{
	pub item: MediaModel,
	pub on_class_changed: Option<EventHandler<MediaClassChangeEvent>>,
	pub on_deleted: Option<EventHandler<String>>,
}

#[component]
pub fn MediaItem(props: MediaItemProps) -> Element
{
	let item = props.item;
	let mtype = item.media_type().as_str_name();
	let filename = item.file_name;
	let id = item.id.unwrap().value;
	let thumb = item.thumb_url;
	let class = item.class;
	let mut class_signal = use_signal(|| match class
	{
		1 => "blur",
		2 => "secret",
		_ => "",
	});
	let url = item.media_url;
	let download = format!("{HOST}{url}");

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
					if class_signal() != "" {
						rsx!{ContextMenuItem {
							index: 2 as usize,
							value: "{id}",
							on_select: move |id: String|{
								spawn(async move {
									if let Ok(_) = set_class(&id, 0).await{
										class_signal.set("");
										if let Some(handler) = props.on_class_changed{
											handler.call(MediaClassChangeEvent { id, class: 0 });
										}
									}
								});
							},
							div{
								class: "contextItem",
								div{
									class: "label",
									"Mark Standard"
								}
							}
						}}
					}else{rsx!{}}
				}
				{
					if class_signal() != "blur" {
						rsx!{ContextMenuItem {
							index: 3 as usize,
							value: "{id}",
							on_select: move |id: String|{
								spawn(async move {
									if let Ok(_) = set_class(&id, 1).await{
										class_signal.set("blur");
										if let Some(handler) = props.on_class_changed{
											handler.call(MediaClassChangeEvent { id, class: 1 });
										}
									}
								});
							},
							div{
								class: "contextItem",
								div{
									class: "label",
									"Mark NSFW"
								}
							}
						}}
					}else{rsx!{}}
				}
				{
					if class_signal() != "secret" {
						rsx!{ContextMenuItem {
							index: 4 as usize,
							value: "{id}",
							on_select: move |id: String|{
								spawn(async move {
									if let Ok(_) = set_class(&id, 2).await{
										class_signal.set("secret");
										if let Some(handler) = props.on_class_changed{
											handler.call(MediaClassChangeEvent { id, class: 2 });
										}
									}
								});
							},
							div{
								class: "contextItem",
								div{
									class: "label",
									"Mark Secret"
								}
							}
						}}
					}else{rsx!{}}
				}
				ContextMenuItem {
					index: 5 as usize,
					value: "{id}",
					on_select: move |id: String|{
						spawn(async move {
							if let Ok(_) = delete_media(id.clone()).await{
								if let Some(handler) = props.on_deleted {
									handler.call(id);
								}
							}
						});
					},
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

#[component]
pub fn MediaItemPlaceHolder() -> Element
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

async fn delete_media(id: String) -> Result<Response<()>, Status>
{
	let mut client = get_rpc_client();
	return client.delete_media(Id { value: id }).await;
}

async fn set_class(id: &String, class: i32) -> Result<Response<()>, Status>
{
	let mut client = get_rpc_client();
	return client
		.set_media_class(SetMediaClassRequest {
			class: class,
			id: Some(Id { value: id.clone() }),
		})
		.await;
}

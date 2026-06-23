use dioxus::{
	html::{
		geometry::{ClientSpace, euclid::Point2D},
		input_data::MouseButton,
	},
	prelude::*,
};
use dioxus_primitives::context_menu::{ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger};
use tonic::{Response, Status};
use web_sys::window;

use crate::{
	HOST,
	contexts::SelectionContext,
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
	pub is_selected: bool,
	pub on_class_changed: Option<EventHandler<MediaClassChangeEvent>>,
	pub on_deleted: Option<EventHandler<String>>,
	pub on_selected: Option<EventHandler<(String, bool, Point2D<f64, ClientSpace>)>>,
	pub bulk_change_class: EventHandler<i32>,
}

#[component]
pub fn MediaItem(props: MediaItemProps) -> Element
{
	let item = props.item.clone();
	let mtype = item.media_type().as_str_name();
	let filename = item.file_name;
	let id = item.id.unwrap().value;
	let thumb = item.thumb_url;
	let class = item.class;
	let class_string = match class
	{
		1 => "blur",
		2 => "secret",
		_ => "",
	};
	let selected_class = match props.is_selected
	{
		true => "selected",
		false => "",
	};
	let url = item.media_url;

	let del_id = id.clone();
	let onmove = move |e: MouseEvent| {
		if e.data().held_buttons().contains(MouseButton::Primary)
		{
			if let Some(handler) = props.on_selected
			{
				let p = e.data().coordinates().client();
				handler.call((del_id.clone(), props.is_selected, p));
			}
		}
	};

	return rsx! {
		ContextMenu{
			ContextMenuTrigger{
				a {
					onmousemove: onmove,
					class: "mediaItem {class_string} {selected_class}",
					href: "{HOST}{url}",
					target: "_blank",
					draggable: false,
					"data-id" : id.clone(),
					img { src: "{HOST}{thumb}", draggable: false }
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
				MediaItemContextMenuItems{
					item: props.item.clone(),
					is_selected: props.is_selected,
					on_class_changed: props.on_class_changed,
					on_deleted: props.on_deleted,
					on_selected: props.on_selected,
					bulk_change_class: props.bulk_change_class
				}
			}
		}
	};
}

#[component]
fn MediaItemContextMenuItems(props: MediaItemProps) -> Element
{
	let item = props.item;
	let id = item.id.unwrap().value;
	let url = item.media_url;
	let download = format!("{HOST}{url}");
	let class = item.class;
	let selection_context: SelectionContext = use_context();
	let selection_count = selection_context.selected_items.len();
	rsx! {
		div{
			"This item"
		}
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
				spawn(async move {
					window().expect("Failed to get window")
						.navigator()
						.clipboard()
						.write_text(&url).await
						.expect("Failed to copy url");
				});
			},
			div{
				class: "contextItem",
				div{
					class: "label",
					"Copy Url"
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
		if class != 0 {
			ContextMenuItem {
				index: 2 as usize,
				value: "{id}",
				on_select: move |id: String|{
					spawn(async move {
						if set_class(&id, 0).await.is_ok(){
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
			}
		}
		if class != 1 {
			ContextMenuItem {
				index: 3 as usize,
				value: "{id}",
				on_select: move |id: String|{
					spawn(async move {
						if set_class(&id, 1).await.is_ok(){
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
			}
		}
		if class != 2 {
			ContextMenuItem {
				index: 4 as usize,
				value: "{id}",
				on_select: move |id: String|{
					spawn(async move {
						if set_class(&id, 2).await.is_ok(){
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
			}
		}
		ContextMenuItem {
			index: 5 as usize,
			value: "{id}",
			on_select: move |id: String|{
				spawn(async move {
					if delete_media(id.clone()).await.is_ok(){
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
		if selection_count > 0{
			div{
				"{selection_count} Selected Items"
			}
			ContextMenuItem {
				index: 6 as usize,
				value: "{id}",
				on_select: move |_id|{
					props.bulk_change_class.call(1);
				},
				div{
					class: "contextItem",
					div{
						class: "label",
						"Mark as NSFW"
					}
				}
			}
			ContextMenuItem {
				index: 7 as usize,
				value: "{id}",
				on_select: move |_id|{
					props.bulk_change_class.call(2);
				},
				div{
					class: "contextItem",
					div{
						class: "label",
						"Mark as Secret"
					}
				}
			}
			ContextMenuItem {
				index: 8 as usize,
				value: "{id}",
				on_select: move |_id|{
					props.bulk_change_class.call(0);
				},
				div{
					class: "contextItem",
					div{
						class: "label",
						"Mark as Standard"
					}
				}
			}
		}
	}
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

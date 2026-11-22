use std::time::Duration;

use super::media_item_context_menu_items::MediaItemContextMenuItems;
use dioxus::{
	html::{
		geometry::{ClientSpace, euclid::Point2D},
		input_data::MouseButton,
	},
	prelude::*,
};
use dioxus_primitives::context_menu::{ContextMenu, ContextMenuContent, ContextMenuTrigger};

use crate::{
	components::{Toolbar, icons::Stack},
	route::Route,
	rpc::aoba::{Dimensions, MediaClass, MediaModel, MediaType},
};
pub type OnItemSelectedEvent = (String, bool);

pub struct MediaClassChangeEvent
{
	pub id: String,
	pub class: MediaClass,
}

#[derive(PartialEq, Clone, Props)]
pub struct MediaItemProps
{
	pub item: MediaModel,
	pub is_selected: bool,
	pub on_class_changed: EventHandler<MediaClassChangeEvent>,
	pub on_deleted: EventHandler<String>,
	pub on_selected: EventHandler<OnItemSelectedEvent>,
	pub bulk_change_class: EventHandler<MediaClass>,
}

const MEDIA_ITEM_CSS: Asset = asset!("/assets/style/mediaItem.scss");

#[component]
pub fn MediaItem(props: MediaItemProps) -> Element
{
	let item = props.item;
	let media_type = item.media_type();
	let class = item.class();
	let class_string = class.to_class_name();
	let thumb = item.main_thumb_url;
	let id = item.id.unwrap_or_default().value;
	let size = item.main_thumb_size.unwrap_or_default();
	let dims = item
		.dimensions
		.map(|d| format!("{}x{}", d.width, d.height))
		.unwrap_or_default();
	let media_url = item.media_url.clone();
	let selected_class = match props.is_selected
	{
		true => "selected",
		false => "",
	};
	rsx! {
		div {
			"data-id" : id.clone(),
			class: "mediaItem {class_string} {selected_class}",
			Toolbar {
				id: id.clone(), media_url, media_class: class, selected: props.is_selected,
				on_deleted: props.on_deleted, on_class_change: props.on_class_changed,
				on_selected: props.on_selected
			}
			Link {
				to: Route::Media { id: id.clone() },
				class: "content",
				MediaThumb { media_type, url: thumb, size }
			}
			div {
				class: "info",
				div {
					class: "title",
					{item.filename}
				}
				div {
					class: "type",
					"{media_type}"
				}
				div  {
					class: "meta",
					{dims}
				}
			}
		}
		document::Link {href: MEDIA_ITEM_CSS, rel: "stylesheet" }
	}
}

const LEGACY_CSS: Asset = asset!("/assets/style/mediaItemLegacy.scss");
#[component]
pub fn MediaItemLegacy(props: MediaItemProps) -> Element
{
	let item = props.item.clone();
	let thumb_type = item.media_type();
	let class = item.class();
	let class_string = class.to_class_name();
	let filename = item.filename;
	let id = item.id.unwrap_or_default().value;
	let thumb = item.main_thumb_url;
	let selected_class = match props.is_selected
	{
		true => "selected",
		false => "",
	};

	let del_id = id.clone();
	let onmove = move |e: MouseEvent| {
		if e.data().held_buttons().contains(MouseButton::Primary)
		{
			props.on_selected.call((del_id.clone(), props.is_selected));
		}
	};

	return rsx! {
		document::Link {href: LEGACY_CSS, rel: "stylesheet"}
		ContextMenu{
			ContextMenuTrigger{
				a {
					key: "{id.clone()}",
					onmousemove: onmove,
					class: "mediaItem {class_string} {selected_class}",
					href: "/media/{id.clone()}",
					// href: Route::Media { id },
					draggable: false,
					"data-id" : id.clone(),
					MediaThumb {
						media_type: thumb_type,
						url: thumb,
						size: item.main_thumb_size.unwrap_or_default()
					}
					// img { src: "{thumb}", draggable: false }
					span { class: "info",
						span { class: "name", {filename.to_string()} }
						span { class: "details",
							span { "{thumb_type}" }
							span { {item.view_count.to_string()} }
						}
					}
				},
			},
			ContextMenuContent{
				MediaItemContextMenuItems{
					item: props.item.clone(),
					on_class_changed: props.on_class_changed,
					on_deleted: props.on_deleted,
					bulk_change_class: props.bulk_change_class
				}
			}
		}
	};
}

#[component]
pub fn MediaThumb(media_type: MediaType, url: String, size: Dimensions) -> Element
{
	match media_type
	{
		MediaType::Video =>
		{
			rsx! {
				video {
					src: url,
					autoplay: true,
					muted: true,
					playsinline: true,
					draggable: false,
					height: size.height,
					width: size.width,
					loop: true
				}
			}
		}
		MediaType::Raw => rsx! { Stack{} },
		MediaType::Text => rsx! { Stack{} },
		MediaType::Code => rsx! { Stack{} },
		_ => rsx! {
			img {
				src: url,
				draggable: false,
				height: size.height,
				width: size.width,
				loading: "lazy"
			}
		},
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

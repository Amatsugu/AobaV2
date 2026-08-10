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
	components::icons::Stack,
	rpc::aoba::{MediaClass, MediaModel, MediaType},
};
pub type OnItemSelectedEvent = (String, bool, Point2D<f64, ClientSpace>);

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
	pub on_selected: Option<EventHandler<OnItemSelectedEvent>>,
	pub bulk_change_class: EventHandler<MediaClass>,
}

#[component]
pub fn MediaItem(props: MediaItemProps) -> Element
{
	let item = props.item.clone();
	let thumb_type = item.media_type();
	let mtype = match item.media_type()
	{
		MediaType::Image => "Image",
		MediaType::Audio => "Audio",
		MediaType::Video => "Video",
		MediaType::Text => "Text",
		MediaType::Code => "Code",
		MediaType::Raw => "Raw",
		_ => "Unknown",
	};
	let class_string = match item.class()
	{
		MediaClass::Nsfw => "blur",
		MediaClass::Secret => "secret",
		_ => "",
	};
	let filename = item.filename;
	let id = item.id.unwrap_or_default().value;
	let thumb = item.thumb_url;
	let selected_class = match props.is_selected
	{
		true => "selected",
		false => "",
	};

	let del_id = id.clone();
	let onmove = move |e: MouseEvent| {
		if e.data().held_buttons().contains(MouseButton::Primary)
			&& let Some(handler) = props.on_selected
		{
			let p = e.data().coordinates().client();
			handler.call((del_id.clone(), props.is_selected, p));
		}
	};

	return rsx! {
		ContextMenu{
			ContextMenuTrigger{
				a {
					onmousemove: onmove,
					class: "mediaItem {class_string} {selected_class}",
					href: "{item.media_url}",
					target: "_blank",
					draggable: false,
					"data-id" : id.clone(),
					MediaThumb { media_type: thumb_type, url: thumb }
					// img { src: "{thumb}", draggable: false }
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
					on_class_changed: props.on_class_changed,
					on_deleted: props.on_deleted,
					bulk_change_class: props.bulk_change_class
				}
			}
		}
	};
}

#[component]
fn MediaThumb(media_type: MediaType, url: String) -> Element
{
	match media_type
	{
		MediaType::Video =>
		{
			rsx! { video { src: url, autoplay: true, muted: true, playsinline: true, draggable: false, loop: true } }
		}
		MediaType::Raw => rsx! { Stack{} },
		_ => rsx! { img { src: url, draggable: false, loading: "lazy" } },
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

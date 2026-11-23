use dioxus::prelude::*;

use crate::{
	components::{Toolbar, icons::Stack},
	contexts::{ThumbnailLoaderContext, ThumbnailWait},
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

#[derive(PartialEq, Clone)]
enum ThumbData
{
	Thumb(String, Dimensions),
	Pending,
	NoThumb,
}

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
	let mut dims = use_signal({
		let d = item.dimensions.clone();
		move || d.map(|d| format!("{}x{}", d.width, d.height)).unwrap_or_default()
	});
	let media_url = item.media_url.clone();
	let selected_class = match props.is_selected
	{
		true => "selected",
		false => "",
	};
	// let mut ctx = init_thumb_loader(id.clone(), item.main_thumb_pending);
	let thumb_data = use_signal({
		let id = id.clone();
		move || {
			// let mut results = ctx.results.write();
			// if let Some(result) = results.iter().find(|r| r.id == id)
			// {
			// 	let result = result.result.clone();
			// 	results.retain(|r| r.id == id);
			// 	if let Some((url, size)) = result
			// 	{
			// 		ThumbData::Thumb(url, size)
			// 	}
			// 	else
			// 	{
			// 		ThumbData::NoThumb
			// 	}
			// }
			// else
			// {
			ThumbData::Thumb(thumb.clone(), size)
			// }
		}
	});
	// use_hook(move || {
	// 	if let ThumbData::Thumb(_, size) = thumb_data.cloned()
	// 	{
	// 		dims.set(format!("{}x{}", size.width, size.height));
	// 	}
	// });
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
				MediaThumb { media_type, data: thumb_data.cloned(), autoplay: class != MediaClass::Nsfw }
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

fn init_thumb_loader(id: String, is_pending: bool) -> ThumbnailLoaderContext
{
	let thumb_ctx = use_context::<ThumbnailLoaderContext>();
	use_effect(move || {
		if is_pending
		{
			thumb_ctx.handle.send(ThumbnailWait {
				id: id.clone(),
				size: crate::rpc::aoba::ThumbnailSize::ResponsiveMedium,
			});
		}
	});
	thumb_ctx
}

#[component]
fn MediaThumb(media_type: MediaType, data: ThumbData, autoplay: bool) -> Element
{
	match data
	{
		ThumbData::Thumb(url, size) => match media_type
		{
			MediaType::Video =>
			{
				rsx! {
					video {
						src: url,
						autoplay,
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
		},
		ThumbData::Pending => rsx! {
			div{
				class: "loading",
				"Loading"
			}
		},
		ThumbData::NoThumb => rsx! {
			div{
				class: "noThumb",
				"No Thumb"
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

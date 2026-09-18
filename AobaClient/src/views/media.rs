use crate::{
	components::{MediaThumb, basic::Button, icons::Cross},
	rpc::{
		aoba::{Id, MediaModel, MediaType, ThumbnailSize},
		get_rpc_client,
	},
	views::media,
};
use dioxus::prelude::*;

const MEDIA_CSS: Asset = asset!("/assets/style/mediaPage.scss");

#[component]
pub fn Media(id: String) -> Element
{
	let media_result = use_resource(use_reactive!(|(id)| async move {
		let mut client = get_rpc_client();
		let result = client.get_media(Id { value: id.clone() }).await;
		if let Ok(item) = result
		{
			let res = item.into_inner();
			res.value
		}
		else
		{
			None
		}
	}));

	media_result
		.cloned()
		.flatten()
		.map_or_else(|| rsx! {"Not Found"}, |media| rsx! {MediaPage { media: media }})
}

#[component]
fn MediaPage(media: MediaModel) -> Element
{
	let media_type = media.media_type();
	let url = media.cdn_url.clone();
	let filename = media.filename.clone();
	rsx! {
		document::Link { rel: "stylesheet", href: MEDIA_CSS }
		div{
			id: "mediaPage",
			div{
				class: "title",
				{filename}
			}
			MediaDisplay { url, media_type }
			div{
				class: "tools"
			}
			MetadataPanel { media: media.clone() }
		}

	}
}

#[component]
fn MetadataPanel(media: MediaModel) -> Element
{
	let cur_class: Signal<String> = use_signal(|| media.class().to_string());
	let media_type = media.media_type();
	let type_name = media_type.to_string();
	rsx! {
		div{
			class: "metadata",
			TagsDisplay { tags: media.tags }
			MetadataEntry { label: "Media Class", {cur_class()} }
			MetadataEntry { label: "Media Type", "{type_name}" }
			MetadataEntry { label: "Views", "{media.view_count}" }
			if let Some(dim) = &media.dimensions{
				MetadataEntry { label: "Dimensions", "{dim}" }
			}
			MetadataEntry {
				label: "Download",
				Button{
					text: "Original"
				}
				if media_type == MediaType::Image{
					Button{
						text: "Webp"
					}
					Button{
						text: "Avif"
					}
					Button{
						text: "Png"
					}
				}
			}
		}
	}
}

#[component]
fn MetadataEntry(label: String, children: Element) -> Element
{
	rsx! {
		div{
			class: "metadataEntry",
			div{
				class: "label",
				{label}
			}
			div{
				class: "value",
				{children}
			}
		}
	}
}

#[component]
fn TagsDisplay(tags: Vec<String>) -> Element
{
	rsx! {
		div{
			class: "tagList",
			for tag in tags {
				Tag {tag}
			}
		}
	}
}

#[component]
fn Tag(tag: String) -> Element
{
	rsx! {
		div {
			class: "tag",
			div{
				class: "label",
				{tag}
			}
			div{
				class: "removeBtn",
				Cross {}
			}
		}
	}
}

#[component]
fn MediaDisplay(url: String, media_type: MediaType) -> Element
{
	rsx! {
		div{
			class: "display",
			MediaThumb{
				media_type,
				url
			}
		}
	}
}

#[component]
fn MultiButton(text: String, onclick: Option<EventHandler<Event<MouseData>>>, children: Element) -> Element
{
	rsx! {
		Button{
			text,
			onclick
		}
		{children}
	}
}

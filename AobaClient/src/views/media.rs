use crate::{
	components::{
		MediaThumb,
		basic::{Button, Input, flex::FlexRow},
		icons::{Cross, NewTabArrowIcon, Stack},
	},
	models::toasts::{ToastCommand, ToastLevel, ToastsContext},
	rpc::{
		aoba::{Id, MediaModel, MediaType, TagUpateRequest},
		get_rpc_client,
	},
};
use dioxus::prelude::*;

const MEDIA_CSS: Asset = asset!("/assets/style/mediaPage.scss");

#[component]
pub fn Media(id: String) -> Element {
	let media_result = use_resource(use_reactive!(|(id)| async move {
		let mut client = get_rpc_client();
		let result = client.get_media(Id { value: id.clone() }).await;
		if let Ok(item) = result {
			let res = item.into_inner();
			res.value
		} else {
			None
		}
	}));

	media_result.cloned().flatten().map_or_else(
		|| rsx! {"Not Found"},
		|media| rsx! {MediaPage { media: media }},
	)
}

#[component]
fn MediaPage(media: MediaModel) -> Element {
	let media_type = media.media_type();
	let url = media.cdn_url.clone();
	let filename = media.filename.clone();
	rsx! {
		document::Link { rel: "stylesheet", href: MEDIA_CSS }
		div{
			id: "mediaPage",
			TitleBar { filename }
			MediaDisplay { url, media_type }
			div{
				class: "tools"
			}
			MetadataPanel { media: media.clone() }
		}

	}
}

#[component]
fn TitleBar(filename: String) -> Element {
	rsx! {
		div {
			class: "titleBar",
			div{
				class: "title",
				{filename}
			}
			div{
				class: "controls",
				Button{
					FlexRow{
						"Open"
						NewTabArrowIcon{}
					}
				}
				Button{
					"Copy Url"
					Stack{}
				}
			}
		}
	}
}

#[component]
fn MetadataPanel(media: MediaModel) -> Element {
	let cur_class: Signal<String> = use_signal(|| media.class().to_string());
	let media_type = media.media_type();
	let type_name = media_type.to_string();
	let toasts_ctx = use_context::<ToastsContext>();
	let media_id = media.id.clone();
	let media_id2 = media.id.clone();
	let mut tags = use_signal(|| media.tags);
	rsx! {
		div{
			class: "metadata",
			TagsDisplay {
				tags: tags.cloned(),
				on_tag_add: move |tag: String| {
					let media_id = media_id.clone();
					spawn(async move {
						let mut client = get_rpc_client();
						match client.add_media_tags(TagUpateRequest{
							media_id,
							tags: vec![tag.clone()]
						}).await {
							Ok(_) => {
								tags.write().push(tag);
							},
							Err(err) => toasts_ctx.push(ToastCommand::Push {
								title: "Failed to add tag".into(),
								message: Some(err.to_string()),
								level: ToastLevel::Error,
								duration: None
							}),
						};
					});
				},
				on_tag_remove: move |tag: String| {
					let media_id = media_id2.clone();
					spawn(async move {
						let mut client = get_rpc_client();
						match client.remove_media_tags(TagUpateRequest{
							media_id,
							tags: vec![tag.clone()]
						}).await {
							Ok(_) => {
								tags.write().retain(|t| t != &tag);
							},
							Err(err) => toasts_ctx.push(ToastCommand::Push {
								title: "Failed to add tag".into(),
								message: Some(err.to_string()),
								level: ToastLevel::Error,
								duration: None
							}),
						};
					});
				}
			}
			MetadataEntry { label: "Media Class", {cur_class()} }
			MetadataEntry { label: "Media Type", {type_name} }
			MetadataEntry { label: "Views", "{media.view_count}" }
			if let Some(dim) = &media.dimensions{
				MetadataEntry { label: "Dimensions", "{dim}" }
			}
			MetadataEntry {
				label: "Download",
				Button{
					"Original"
				}
				if media_type == MediaType::Image{
					Button{
						"Webp"
					}
					Button{
						"Avif"
					}
					Button{
						"Png"
					}
				}
			}
		}
	}
}

#[component]
fn MetadataEntry(label: String, children: Element) -> Element {
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
fn TagsDisplay(
	tags: Vec<String>,
	on_tag_add: EventHandler<String>,
	on_tag_remove: EventHandler<String>,
) -> Element {
	let mut tag: Signal<String> = use_signal(|| "".into());

	rsx! {
		div{
			class: "tagList",
			for tag in tags {
				Tag {
					tag,
					on_remove: move |tag|{
						on_tag_remove.call(tag);
					}
				}
			}
			input{
				type: "text",
				name: "tag",
				value: tag.cloned(),
				placeholder: "Add Tags",
				onkeyup: move |e: Event<KeyboardData>| {
					if e.key() == Key::Enter {
						on_tag_add.call(tag.cloned());
						tag.set("".into());
					}
				},
				oninput: move |e: Event<FormData>|{
					tag.set(e.value());
				}
			}
		}
	}
}

#[component]
fn Tag(tag: String, on_remove: EventHandler<String>) -> Element {
	rsx! {
		div {
			class: "tag",
			div {
				class: "label",
				{tag.clone()}
			}
			div {
				class: "removeBtn",
				onclick: move |_|{
					let tag = tag.clone();
					on_remove.call(tag);
				},
				Cross {}
			}
		}
	}
}

#[component]
fn MediaDisplay(url: String, media_type: MediaType) -> Element {
	rsx! {
		div{
			class: "display",
			MediaThumb {
				media_type,
				url
			}
		}
	}
}

#[component]
fn MultiButton(
	text: String,
	onclick: Option<EventHandler<Event<MouseData>>>,
	children: Element,
) -> Element {
	rsx! {
		Button{
			onclick,
			{text}
		}
		{children}
	}
}

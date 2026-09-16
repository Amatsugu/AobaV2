use crate::{
	components::{MediaThumb, basic::Button},
	rpc::{
		aoba::{Id, MediaModel},
		get_rpc_client,
	},
};
use dioxus::prelude::*;

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
	use crate::rpc::aoba::MediaClass;
	let class = media.class();
	let media_type = media.media_type();
	let url = media.thumb_url;
	let cur_class = use_signal(|| match class {
		MediaClass::Unspecified => "Unkown",
		MediaClass::Standard => "Standard",
		MediaClass::Nsfw => "NSFW",
		MediaClass::Secret => "Secret",
	});
	rsx! {
		MediaThumb{
			media_type,
			url
		}
		label { "Media Class: {cur_class()}" }

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
			text,
			onclick
		}
		{children}
	}
}

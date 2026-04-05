use crate::HOST;
use crate::rpc::{
	aoba::{Id, MediaModel},
	get_rpc_client,
};
use dioxus::prelude::*;

#[component]
pub fn Media(id: String) -> Element {
	let media_result = use_resource(use_reactive!(|(id)| async move {
		let mut client = get_rpc_client();
		let result = client.get_media(Id { value: id.clone() }).await;
		if let Ok(item) = result {
			let res = item.into_inner();
			return res.value;
		} else {
			return None;
		}
	}));

	return match media_result.cloned().unwrap_or(None) {
		Some(media) => {
			return rsx! {MediaPage{media: media}};
		}
		None => rsx! {"Not Found"},
	};
}

#[component]
fn MediaPage(media: MediaModel) -> Element {
	let url = media.thumb_url;
	// let id = media.id.expect("Media has no id").value.clone();
	let cur_class = use_signal(|| match media.class {
		0 => "Standard",
		1 => "NSFW",
		2 => "Secret",
		_ => "Unkown",
	});
	rsx! {
		img { src: "{HOST}{url}",  }
		label { "Media Class: {cur_class()}" }

	}
}

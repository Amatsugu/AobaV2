use dioxus::prelude::*;
use tonic::IntoRequest;

use crate::{
	components::MediaItem,
	rpc::{aoba::PageFilter, get_rpc_client},
};

#[derive(PartialEq, Clone, Props)]
pub struct MediaGridProps {
	pub query: Option<String>,
	#[props(default = Some(1))]
	pub page: Option<i32>,
	#[props(default = Some(100))]
	pub page_size: Option<i32>,
}

impl IntoRequest<PageFilter> for MediaGridProps {
	fn into_request(self) -> tonic::Request<PageFilter> {
		let f: PageFilter = self.into();
		f.into_request()
	}
}

impl Into<PageFilter> for MediaGridProps {
	fn into(self) -> PageFilter {
		PageFilter {
			page: self.page,
			page_size: self.page_size,
			query: self.query,
		}
	}
}

#[component]
pub fn MediaGrid(props: MediaGridProps) -> Element {
	let media_result = use_resource(use_reactive!(|(props)| async move {
		let mut client = get_rpc_client();
		let result = client.list_media(props.into_request()).await;
		return result.expect("Failed to load media").into_inner();
	}));

	match &*media_result.read_unchecked() {
		Some(result) => rsx! {
			div{
				class: "mediaGrid",
				{result.items.iter().map(|itm| rsx!{
					MediaItem { item: itm.clone() }
				})},
			}
		},
		None => rsx! {
			div{
				class: "mediaGrid",
				div {
					"No results could be loaded"
				}
			}
		},
	}
}

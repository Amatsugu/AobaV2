use std::str::FromStr;

use dioxus::prelude::*;
use tonic::{metadata::MetadataValue, IntoRequest, Request};

use crate::rpc::{
	aoba::{MediaModel, PageFilter},
	get_rpc_client,
};

#[derive(PartialEq, Clone, Props)]
pub struct MediaGridProps {
	pub query: Option<String>,
	pub page: Option<i32>,
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
	let media_result = use_resource(|| async move {
		let mut client = get_rpc_client();
		let mut req = Request::new(PageFilter::default());
		req.metadata_mut()
			.insert("authorization", "Bearer <toto: get token>".parse().unwrap());
		let result = client.list_media(req).await;
		return result.expect("Failed to load media").into_inner();
	});

	match &*media_result.read_unchecked() {
		Some(result) => rsx! {
			div{
				class: "mediaGrid",
				{result.items.iter().map(|itm| rsx!{
					MediaItem { item: itm.clone() }
				})}
			}
		},
		None => rsx!(),
	}
	// let items = media_result..unwrap().items;
	// rsx! {
	// 	div{
	// 		class: "mediaGrid",
	// 		{items.iter().map(|itm| rsx!{
	// 			MediaItem { item: itm.clone() }
	// 		})}
	// 	}
	// }
}

#[derive(PartialEq, Clone, Props)]
pub struct MediaItemProps {
	pub item: MediaModel,
}

#[component]
pub fn MediaItem(props: MediaItemProps) -> Element {
	let filename = props.item.file_name;
	let id = props.item.id.unwrap().value;
	let mtype = props.item.media_type.to_string();
	// let url = "https://aoba.app/i/{}";
	rsx! {
		div{
			class: "mediaItem",
			img{ src: "https://aoba.app/i/{id}" }
			div {
				class: "info",
				span{
					class: "name",
					"{filename}"
				},
				div{
					class: "details",
					span{
						"{mtype}"
					},
					span{
						"{props.item.view_count}"
					},
				}
			}
		}
	}
}

use dioxus::prelude::*;
use tonic::IntoRequest;

use crate::{
	components::MediaItem,
	rpc::{aoba::PageFilter, get_rpc_client},
};

#[derive(PartialEq, Clone, Props)]
pub struct MediaGridProps {
	pub query: Option<String>,
	pub max_page: Signal<i32>,
	pub total_items: Signal<i32>,
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
pub fn MediaGrid(mut props: MediaGridProps) -> Element {
	let media_result = use_resource(use_reactive!(|(props)| async move {
		let mut client = get_rpc_client();
		let result = client.list_media(props.into_request()).await;
		if let Ok(items) = result {
			let res = items.into_inner();
			return Ok(res);
		} else {
			let err = result.err().unwrap();
			let message = err.message();
			return Err(format!("Failed to load results: {message}"));
		}
	}));

	match media_result.cloned() {
		Some(value) => match value {
			Ok(result) => {
				let pagination = result.pagination.unwrap();
				let total_pages = pagination.total_pages;
				let total_items = pagination.total_items;
				props.max_page.set(total_pages.max(1));
				props.total_items.set(total_items.max(1));
				return rsx! {
					div {
						class: "mediaGrid",
						// oncontextmenu: oncontext,
						{result.items.iter().map(|itm| rsx!{
							MediaItem {
								item: itm.clone()
							}
						})},
					}
				};
			}
			Err(msg) => rsx! {
				div {
					class: "mediaGrid",
					div {
						"Failed to load results: {msg}"
					}
				}
			},
		},
		None => rsx! {
			div{
				class: "mediaGrid",
				{(0..50).map(|_| rsx!{
					MediaItem { }
				})}
			}
		},
	}
}

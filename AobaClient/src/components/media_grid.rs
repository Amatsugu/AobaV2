use dioxus::prelude::*;
use tonic::IntoRequest;

use crate::{
	components::{ContextMenu, ContextMenuItem, MediaItem},
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
		if let Ok(items) = result {
			return Ok(items.into_inner());
		} else {
			let err = result.err().unwrap();
			let message = err.message();
			return Err(format!("Failed to load results: {message}"));
		}
	}));

	let mut context_menu: Signal<Element> = use_signal(|| rsx! {});
	let oncontext = move |event: Event<MouseData>| {
		event.prevent_default();
		let data = event.data();
		let pos = data.coordinates().client();
		let left = pos.x;
		let top = pos.y;
		context_menu.set(rsx! {
			ContextMenu{
				left: left,
				top: top,
				items: vec![
					rsx!{
						ContextMenuItem{
							name: "Details"
						}
					},
					rsx!{
						ContextMenuItem{
							name: "Download"
						}
					},
					rsx!{
						ContextMenuItem{
							name: "Delete"
						}
					},
				]
			}
		});
	};

	match media_result.cloned() {
		Some(value) => match value {
			Ok(result) => rsx! {
				div {
					class: "mediaGrid",
					onclick: move |_e| {
						context_menu.set(rsx!{});
					},
					{result.items.iter().map(|itm| rsx!{
						MediaItem { item: Some(itm.clone()), oncontextmenu: oncontext }
					})},
				}
				{context_menu.cloned()}
			},
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
					MediaItem {}
				})}
			}
		},
	}
}

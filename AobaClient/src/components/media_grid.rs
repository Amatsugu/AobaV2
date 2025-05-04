use dioxus::prelude::*;
use tonic::{IntoRequest, Request};

use crate::{
	components::MediaItem,
	contexts::AuthContext,
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
	let jwt = use_context::<AuthContext>().jwt;
	let media_result = use_resource(use_reactive!(|(props, jwt)| async move {
		let mut client = get_rpc_client();
		let mut req = Request::new(props.into());
		let token = if jwt.cloned().is_some() {
			jwt.unwrap()
		} else {
			"".into()
		};
		req.metadata_mut()
			.insert("authorization", format!("Bearer {token}").parse().unwrap());
		let result = client.list_media(req).await;
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

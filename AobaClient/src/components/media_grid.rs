use dioxus::prelude::*;

use crate::{
	components::{MediaItem, MediaItemPlaceHolder},
	rpc::{
		aoba::{MediaModel, PageFilter},
		get_rpc_client,
	},
};

#[derive(PartialEq, Clone, Props)]
pub struct MediaGridProps
{
	pub query: Signal<String>,
	pub max_page: Signal<i32>,
	pub total_items: Signal<i32>,
	pub page: Signal<i32>,
	pub page_size: Signal<i32>,
}

#[component]
pub fn MediaGrid(mut props: MediaGridProps) -> Element
{
	let mut error_display = use_signal(|| {
		rsx! {}
	});
	let mut items = use_signal::<Option<Vec<MediaModel>>>(|| None);
	let media_result = use_resource(use_reactive!(|(props)| async move {
		items.set(None);
		let mut client = get_rpc_client();
		let request = PageFilter {
			page_size: Some(props.page_size.cloned()),
			page: Some(props.page.cloned()),
			query: Some(props.query.cloned()),
		};
		let result = client.list_media(request).await;
		if let Ok(items) = result
		{
			let res = items.into_inner();

			return Ok(res);
		}
		else
		{
			let err = result.err().unwrap();
			let message = err.message();
			return Err(format!("Failed to load results: {message}"));
		}
	}));

	use_effect(move || match media_result()
	{
		Some(value) => match value
		{
			Ok(result) =>
			{
				if let Some(pagination) = result.pagination
				{
					let total_pages = pagination.total_pages;
					let total_items = pagination.total_items;
					props.max_page.set(total_pages.max(1));
					props.total_items.set(total_items.max(1));
				}
				items.set(Some(result.items));
				error_display.set(rsx! {});
			}
			Err(msg) => error_display.set(rsx! {
				div{
					"Failed to load results: {msg}"
				}
			}),
		},
		_ =>
		{}
	});

	rsx! {
		div {
			class: "mediaGrid",
			{error_display}
			{match items(){
				Some(itms) => rsx!{MediaList { items: itms }},
				None => rsx!{PlaceholderGrid { count: props.page_size.cloned() as usize }}
			}}
		}
	}
}

#[component]
fn PlaceholderGrid(count: usize) -> Element
{
	rsx! {
		div{
			class: "mediaGrid",
			{(0..count).map(|_| rsx!{
				MediaItemPlaceHolder { }
			})}
		}
	}
}

#[component]
fn MediaList(items: Vec<MediaModel>) -> Element
{
	rsx! {
		{items.iter().map(|itm| rsx!{
			MediaItem {
				item: itm.clone()
			}
		})}
	}
}

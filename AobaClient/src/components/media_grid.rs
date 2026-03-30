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
	let media_result = use_resource(use_reactive!(|(props)| async move {
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

	let mut media_grid_display = use_signal(|| {
		rsx! {
			div{
				class: "mediaGrid",
				{(0..50).map(|_| rsx!{
					MediaItemPlaceHolder { }
				})}
			}
		}
	});
	let mut items = use_signal::<Vec<MediaModel>>(|| Vec::new());

	use_memo(move || match media_result()
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
				items.set(result.items);
				media_grid_display.set(rsx! {
					MediaList { items }
				});
			}
			Err(msg) => media_grid_display.set(rsx! {
				div{
					"Failed to load results: {msg}"
				}
			}),
		},
		_ => (),
	});

	rsx! {
		div {
			class: "mediaGrid",
			{media_grid_display}
		}
	}
}

#[component]
fn MediaList(items: Signal<Vec<MediaModel>>) -> Element
{
	let vec = items.cloned();
	rsx! {
		{vec.iter().map(|itm| rsx!{
			MediaItem {
				item: itm.clone()
			}
		})}
	}
}

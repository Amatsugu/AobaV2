use dioxus::prelude::*;
use tonic::{Response, Status};

use crate::{
	components::{MediaClassChangeEvent, MediaItem, MediaItemPlaceHolder, OnItemSelectedEvent},
	rpc::{
		aoba::{Id, MediaClass, MediaModel, PageFilter, SetMediaClassRequest},
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
	pub selected_items: Vec<String>,
	pub on_page_loaded: Option<EventHandler<PaginationInfo>>,
	pub on_item_selected: Option<EventHandler<OnItemSelectedEvent>>,
	pub onmouseup: EventHandler<MouseEvent>,
	pub onmousedown: EventHandler<MouseEvent>,
	pub bulk_change_class: EventHandler<MediaClass>,
}

pub struct PaginationInfo
{
	pub total_pages: i32,
	pub total_items: i32,
}

#[component]
pub fn MediaGrid(props: MediaGridProps) -> Element
{
	let mut error_display = use_signal(|| {
		rsx! {}
	});
	let mut items = use_signal::<Option<Vec<MediaModel>>>(|| None);
	let media_result = use_resource(use_reactive!(|(props)| async move {
		let mut client = get_rpc_client();
		let request = PageFilter {
			page_size: Some(props.page_size.cloned()),
			page: Some(props.page.cloned()),
			query: Some(props.query.cloned()),
		};
		match client.list_media(request).await
		{
			Ok(items) => Ok(items.into_inner()),
			Err(err) => Err(format!("Failed to load results: {}", err.message())),
		}
	}));

	use_effect(move || {
		if let Some(value) = media_result()
		{
			match value
			{
				Ok(result) =>
				{
					if let Some(pagination) = result.pagination
					{
						let total_pages = pagination.total_pages;
						let total_items = pagination.total_items;
						if let Some(handler) = props.on_page_loaded
						{
							handler.call(PaginationInfo {
								total_pages,
								total_items,
							});
						}
					}
					items.set(Some(result.items));
					error_display.set(rsx! {});
				}
				Err(msg) => error_display.set(rsx! {
					div{
						"Failed to load results: {msg}"
					}
				}),
			}
		}
	});

	rsx! {
		div {
			class: "mediaGrid",
			onmouseup: props.onmouseup,
			onmousedown: props.onmousedown,
			{error_display}
			{match items(){
				Some(itms) => rsx!{
					MediaList {
						items: itms,
						selected: props.selected_items,
						bulk_change_class: props.bulk_change_class,
						on_item_selected: props.on_item_selected,
						on_item_deleted: move |id: String|{
							spawn(async move {
								if delete_media(id.clone()).await.is_ok() &&
									 let Some(cur) = items.cloned(){
										let filtered = cur.iter()
											.filter(|i| i.id.as_ref().map(|i| i.value == id).unwrap_or_default())
											.cloned()
											.collect();
										items.set(Some(filtered));
								}
							});
						},
						on_class_changed: move |e: MediaClassChangeEvent|{
							spawn(async move {
								if set_class(&e.id, e.class).await.is_ok()
									&& let Some(cur) = items.cloned() {
										let updated = cur.iter()
											.map(|i|{
												let mut itm = i.clone();
												if itm.id.as_ref().map(|id| id.value == e.id).unwrap_or_default() {
													itm.class = e.class as i32;
												}
												itm
											})
											.collect();
										info!("Class changed");
										items.set(Some(updated));
								}
							});
						}
					}
				},
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
fn MediaList(
	items: Vec<MediaModel>,
	selected: Vec<String>,
	on_item_deleted: EventHandler<String>,
	on_item_selected: Option<EventHandler<OnItemSelectedEvent>>,
	on_class_changed: EventHandler<MediaClassChangeEvent>,
	bulk_change_class: EventHandler<MediaClass>,
) -> Element
{
	rsx! {
		{items.iter().map(|itm| {
			let is_selected = itm.id.as_ref().map(|id| selected.contains(&id.value)).unwrap_or_default();
			rsx!{
				MediaItem {
					item: itm.clone(),
					is_selected,
					on_deleted: on_item_deleted,
					on_selected: on_item_selected,
					on_class_changed: on_class_changed,
					bulk_change_class
				}
			}
		})}
	}
}

async fn delete_media(id: String) -> Result<Response<()>, Status>
{
	let mut client = get_rpc_client();
	return client.delete_media(Id { value: id }).await;
}

async fn set_class(id: &str, class: MediaClass) -> Result<Response<()>, Status>
{
	let mut client = get_rpc_client();
	return client
		.set_media_class(SetMediaClassRequest {
			class: class.into(),
			id: Some(Id { value: id.to_owned() }),
		})
		.await;
}

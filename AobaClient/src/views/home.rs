use crate::{
	components::{MediaGrid, Pagination, PaginationInfo, Search, SelectionBar, UploadArea},
	contexts::SelectionContext,
	rpc::{
		aoba::{Id, IdList, SetMediaClassBulkRequest},
		get_rpc_client,
	},
};
use dioxus::{
	html::{
		geometry::{ClientSpace, euclid::Point2D},
		input_data::MouseButton,
	},
	prelude::*,
};

#[derive(Debug, Clone)]
enum SelectionPhase
{
	Start,
	Selecting,
	Idle,
}

#[derive(Debug, Clone)]
enum SelectionMode
{
	Add,
	Remove,
}

const MIN_DRAG_DISTANCE: f64 = 8.0;

#[component]
pub fn Home(page: Option<i32>, q: Option<String>) -> Element
{
	let mut query = use_signal(|| q.unwrap_or("".to_string()));
	let mut page = use_signal(|| page.unwrap_or(1));
	let page_size = use_signal::<i32>(|| 100);
	let mut max_page = use_signal(|| 1 as i32);
	let mut item_count = use_signal(|| 0 as i32);
	let mut selection_context = use_context_provider(|| SelectionContext::default());
	let mut last_pos = use_signal(|| None::<Point2D<f64, ClientSpace>>);
	// let mut selected_items: Signal<Vec<String>> = use_signal(|| Vec::new());
	let mut seletion_mode: Signal<SelectionMode> = use_signal(|| SelectionMode::Add);
	let mut seletion_phase: Signal<SelectionPhase> = use_signal(|| SelectionPhase::Start);
	rsx! {
		div	{
			class: "stickyTop",
			Search {
				query: query(),
				oninput: move |q| {
					query.set(q);
					page.set(1);
				},
				onchange: move |_|{
					router().push(format!("/?page={}&q={}", page(), query()));
				}
			},
			Pagination {
				page, max_page, item_count,
				on_page_change: move |p|{
					page.set(p);
					router().push(format!("/?page={}&q={}", page(), query()));
				}
			},
		}
		UploadArea{
			MediaGrid {
				query: query,
				page: page,
				max_page,
				total_items: item_count,
				selected_items: selection_context.selected_items.cloned(),
				page_size,
				on_page_loaded: move |p: PaginationInfo| {
					max_page.set(p.total_pages);
					item_count.set(p.total_items);
				},
				on_item_selected: move |select: (String, bool, Point2D<f64, ClientSpace>)| {
					let mut items = selection_context.selected_items.cloned();
					let (id, selected, pos) = select;
					let delta = if let Some(last_pos) = last_pos.cloned() {
						(last_pos - pos).length()
					}else{
						0.0
					};
					last_pos.set(Some(pos));
					if delta <= MIN_DRAG_DISTANCE {
						return;
					}
					match seletion_phase.cloned(){
						SelectionPhase::Start => {
							let mode = match selected {
								true => SelectionMode::Remove,
								false => SelectionMode::Add
							};
							process_selection(&mut items, mode.clone(), id.clone());
							seletion_mode.set(mode);
							seletion_phase.set(SelectionPhase::Selecting);
						},
						SelectionPhase::Selecting => {
							let mode = seletion_mode.cloned();
							process_selection(&mut items, mode, id.clone());
						},
						SelectionPhase::Idle => (),
					}

					selection_context.selected_items.set(items);
				},
				bulk_change_class,
				onmouseup: move |e: MouseEvent|{
					if let Some(button) = e.data().trigger_button()
					{
						if button == MouseButton::Primary{
							seletion_phase.set(SelectionPhase::Idle);
							last_pos.set(None);
						}
					}

				},
				onmousedown: move |e: MouseEvent|{
					if let Some(button) = e.data().trigger_button()
					{
						if button == MouseButton::Primary{
							seletion_phase.set(SelectionPhase::Start);
						}
					}
				},
			}
			SelectionBar{
				selected_items: selection_context.selected_items.cloned(),
				on_selection_cleared: move |_|{
					selection_context.selected_items.set(Vec::new());
				},
				on_items_delete: move |_|{
					spawn(async move {
						let mut client = get_rpc_client();
						let item_ids = selection_context.selected_items.cloned().iter().map(|id| Id { value: id.clone() }).collect();
						let req = IdList{
							value: item_ids
						};
						if let Err(err) = client.delete_media_bulk(req).await {
							error!("Failed to delete items: {:?}", err);
						}
						query.set(query.cloned());
						selection_context.selected_items.set(Vec::new());
					});
				}
			}
		}
	}
}

fn process_selection(items: &mut Vec<String>, mode: SelectionMode, id: String)
{
	match mode
	{
		SelectionMode::Add =>
		{
			if !items.contains(&id)
			{
				items.push(id.clone());
			}
		}
		SelectionMode::Remove =>
		{
			*items = items.iter().filter(|i| *i != &id).map(|i| i.clone()).collect();
		}
	}
}

fn bulk_change_class(class: i32)
{
	spawn(async move {
		let mut client = get_rpc_client();
		let mut selection_context: SelectionContext = use_context();
		info!("Changing class to {}", class);
		let ids = selection_context
			.selected_items
			.cloned()
			.iter()
			.map(|id| Id { value: id.clone() })
			.collect();
		if client
			.set_media_class_bulk(SetMediaClassBulkRequest {
				class,
				ids: Some(IdList { value: ids }),
			})
			.await
			.is_err()
		{
			error!("Failed to bulk change class for items")
		}

		selection_context.selected_items.set(Vec::new());
	});
}

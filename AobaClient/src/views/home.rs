use crate::components::{MediaGrid, Pagination, PaginationInfo, Search, SelectionBar};
use dioxus::{html::input_data::MouseButton, prelude::*};

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

#[component]
pub fn Home(page: Option<i32>, q: Option<String>) -> Element
{
	let mut query = use_signal(|| q.unwrap_or("".to_string()));
	let mut page = use_signal(|| page.unwrap_or(1));
	let page_size = use_signal::<i32>(|| 100);
	let mut max_page = use_signal(|| 1 as i32);
	let mut item_count = use_signal(|| 0 as i32);
	let mut selected_items: Signal<Vec<String>> = use_signal(|| Vec::new());
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
		MediaGrid {
			query: query,
			page: page,
			max_page,
			total_items: item_count,
			selected_items: selected_items.cloned(),
			page_size,
			on_page_loaded: move |p: PaginationInfo| {
				max_page.set(p.total_pages);
				item_count.set(p.total_items);
			},
			on_item_selected: move |select: (String, bool)| {
				let mut items = selected_items.cloned();
				let (id, selected) = select;
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

				selected_items.set(items);
			},
			onmouseup: move |e: MouseEvent|{
				if let Some(button) = e.data().trigger_button()
				{
					if button == MouseButton::Primary{
						seletion_phase.set(SelectionPhase::Idle);
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
			selected_items: selected_items.cloned(),
			on_selection_cleared: move |_|{
				selected_items.set(Vec::new());
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

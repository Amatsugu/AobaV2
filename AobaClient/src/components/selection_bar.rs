use dioxus::prelude::*;

use crate::components::{
	Modal,
	basic::{Button, ButtonVariant},
};

#[component]
pub fn SelectionBar(
	selected_items: Vec<String>,
	on_selection_cleared: EventHandler,
	on_items_delete: EventHandler,
) -> Element
{
	let mut delete_modal_open = use_signal(|| false);
	if selected_items.len() == 0
	{
		return rsx! {};
	}
	else
	{
		let selection_count = selected_items.len();
		return rsx! {
			div{
				id: "selectionBarContainer",
				div{
					id: "selectionBar",
					div{
						class: "info",
						"{selection_count} Items Selected"
					},
					div{
						class: "controls",
						BarButton{
							onclick: move|_|{},
							"Mark As"
						}
						BarButton{
							onclick: move|_|{
								delete_modal_open.set(true);
							},
							"Delete"
						}
						BarButton{
							onclick: move |_|{
								on_selection_cleared.call(());
							},
							span{
								"Deselect All"
							}
						}
					}
				}
			}
			Modal{
				title: "Confirm Deletion of {selection_count} items",
				is_open: delete_modal_open(),
				div{
					class: "vertFlexBox",
					"Are you sure you want to delete these items? This cannot be undone!",
					div{
						class: "horizFlexBox center",
						style: "gap:10px",
						Button{
							text: "Delete",
							onclick: move |_|{
								delete_modal_open.set(false);
								on_items_delete.call(());
							}
						}
						Button{
							text: "Cancel",
							variant: ButtonVariant::Cancel,
							onclick: move |_|{
								delete_modal_open.set(false);
							}
						}
					}
				}
			}
		};
	}
}

#[component]
pub fn BarButton(onclick: EventHandler<MouseEvent>, children: Element) -> Element
{
	rsx! {
		div{
			class: "button",
			onclick:onclick,
			{children}
		}
	}
}

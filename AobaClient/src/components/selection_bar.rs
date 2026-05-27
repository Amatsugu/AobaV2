use crate::HOST;
use dioxus::prelude::*;
use web_sys::window;

use crate::components::{
	Modal,
	basic::{Button, ButtonVariant},
};

const CLEAR_ICON: Asset = asset!("/assets/icons/clear.svg");
const TRASH_ICON: Asset = asset!("/assets/icons/trash.svg");
const TAG_ICON: Asset = asset!("/assets/icons/tag.svg");
const COPY_ICON: Asset = asset!("/assets/icons/copy-doc.svg");

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
							onclick: move|_|{
								let item_ids = selected_items.clone();
								spawn(async move {
									let links : Vec<String> = item_ids.iter()
										.map(|id| format!("{HOST}/m/{id}"))
										.collect();
									let joined = links.join("\n");
									match window().expect("Failed to get window").navigator().clipboard().write_text(joined.as_str()).await {
										Ok(_) => (),
										Err(_) => error!("Failed to write to clipboard"),
									};
								});
							},
							img{
								src: COPY_ICON
							}
						}
						BarButton{
							onclick: move|_|{},
							img{
								src: TAG_ICON
							}
						}
						BarButton{
							onclick: move|_|{
								delete_modal_open.set(true);
							},
							img{
								src: TRASH_ICON
							}
						}
						BarButton{
							onclick: move |_|{
								on_selection_cleared.call(());
							},
							img{
								src: CLEAR_ICON
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

use dioxus::prelude::*;

#[component]
pub fn SelectionBar(selected_items: Vec<String>, on_selection_cleared: EventHandler) -> Element
{
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
						Button{
							onclick: move|_|{},
							"Mark As"
						}
						Button{
							onclick: move|_|{},
							"Delete"
						}
						Button{
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
		};
	}
}

#[component]
pub fn Button(onclick: EventHandler<MouseEvent>, children: Element) -> Element
{
	rsx! {
		div{
			class: "button",
			onclick:onclick,
			{children}
		}
	}
}

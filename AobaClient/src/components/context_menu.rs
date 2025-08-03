use core::str;

use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ContextMenuProps {
	pub top: f64,
	pub left: f64,
	pub items: Option<Vec<Element>>,
}

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
	let menu_items = if let Some(items) = props.items {
		rsx! {
			ItemList { items }
		}
	} else {
		rsx! {}
	};

	rsx! {
		div {
			class: "contextMenu",
			style: "left: {props.left}px; top: {props.top}px;",
			{menu_items}
		}
	}
}

#[component]
fn ItemList(items: Vec<Element>) -> Element {
	rsx! {
		div{
			class: "itemList",
			{items.iter().map(|e| rsx!{{e}}) }
		}
	}
}

#[derive(PartialEq, Clone, Props)]
pub struct ContextMenuItemProps {
	pub name: String,
	pub sub_items: Option<Vec<Element>>,
}

#[component]
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element {
	rsx! {
		div{
			class: "contextItem",
			div {
				class: "icon"
			},
			div {
				class: "label",
				{props.name}
			}
		}
	}
}

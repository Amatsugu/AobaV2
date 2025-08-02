use core::str;

use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ContextMenuProps {
	pub top: f64,
	pub left: f64,
	pub items: Option<Vec<()>>,
}

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
	rsx! {
		div {
			style: "background:#000; position: absolute; left: {props.left}px; top: {props.top}px;",
			"Contect Menu"
		}
	}
}

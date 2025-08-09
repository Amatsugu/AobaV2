use core::str;

use dioxus::prelude::*;

pub mod props {
	use dioxus::prelude::*;

	#[derive(PartialEq, Clone, Props)]
	pub struct ContextMenu {
		pub top: f64,
		pub left: f64,
		pub items: Option<Vec<ContextMenuItem>>,
	}

	#[derive(PartialEq, Clone, Props, Default)]
	pub struct ContextMenuItem {
		pub name: String,
		pub sub_items: Option<Vec<ContextMenuItem>>,
	}
}

#[derive(Clone, Copy, Default)]
pub struct ContextMenuRenderer {
	pub menu: Signal<Option<props::ContextMenu>>,
}

impl ContextMenuRenderer {
	pub fn close(&mut self) {
		self.menu.set(None);
	}

	pub fn render(&self) -> Element {
		if let Some(menu) = self.menu.cloned() {
			rsx! {
				ContextMenu{
					items: menu.items,
					left: menu.left,
					top: menu.top
				}
			}
		} else {
			rsx! {}
		}
	}
}

#[component]
pub fn ContextMenuRoot() -> Element {
	let renderer = use_context::<ContextMenuRenderer>();
	rsx! {
		{renderer.render()}
	}
}

#[component]
fn ContextMenu(props: props::ContextMenu) -> Element {
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
fn ItemList(items: Vec<props::ContextMenuItem>) -> Element {
	rsx! {
		div{
			class: "itemList",
			{items.iter().map(|e| rsx!{
				ContextMenuItem{
					name: e.name.clone(),
					sub_items: e.sub_items.clone()
				}
			})}
		}
	}
}

#[component]
fn ContextMenuItem(props: props::ContextMenuItem) -> Element {
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

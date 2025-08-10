use core::str;

use dioxus::prelude::*;

mod props {
	use dioxus::prelude::*;

	#[derive(PartialEq, Clone, Props)]
	pub struct ContextMenu {
		pub top: f64,
		pub left: f64,
		pub items: Element,
	}

	#[derive(PartialEq, Clone, Props, Default)]
	pub struct ContextMenuItem {
		pub name: String,
		pub sub_items: Option<Element>,
		pub onclick: Option<EventHandler<MouseEvent>>,
	}
}

#[derive(Clone, Copy, Default)]
pub struct ContextMenuRenderer {
	pub menu: Signal<Option<Element>>,
}

impl ContextMenuRenderer {
	pub fn close(&mut self) {
		self.menu.set(None);
	}

	pub fn render(&self) -> Element {
		if let Some(menu) = self.menu.cloned() {
			rsx! {
				{menu}
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
pub fn ContextMenu(props: props::ContextMenu) -> Element {
	rsx! {
		div {
			class: "contextMenu",
			style: "left: {props.left}px; top: {props.top}px;",
			ItemList { items: props.items }
		}
	}
}

#[component]
fn ItemList(items: Element) -> Element {
	rsx! {
		div{
			class: "itemList",
			{items}
		}
	}
}

#[component]
pub fn ContextMenuItem(props: props::ContextMenuItem) -> Element {
	let mut renderer = use_context::<ContextMenuRenderer>();
	if let Some(_sub) = props.sub_items {
		todo!("Sub Menu");
	}
	rsx! {
		div{
			onclick: move |e|{
				if let Some(handler) = props.onclick{
					handler.call(e);
				}
				renderer.close();
			},
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

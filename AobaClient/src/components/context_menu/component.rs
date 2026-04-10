use super::props::*;
use dioxus::prelude::*;

const CONTEXT_MENU_CSS: Asset = asset!("./style.scss");

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element
{
	rsx! {
		document::Link { rel: "stylesheet", href: CONTEXT_MENU_CSS }
		{props.children}
	}
}

#[component]
pub fn ContextMenuTrigger(props: ContextMenuTriggerProps) -> Element
{
	rsx! {
		div{
			class: "contextMenuTrigger",
			oncontextmenu: move|e|{

			},
			{props.children}
		}
	}
}

#[component]
pub fn ContextMenuContent(props: ContextMenuContentProps) -> Element
{
	rsx! {
		div{
			class: "contextMenuContent",
			{props.children}
		}
	}
}

#[component]
pub fn ContextMenuItem(props: ContextMenuItemProps) -> Element
{
	rsx! {
		div {
			class: "contextMenuItem",
			onclick: move |_|{
				props.on_select.call(props.value.clone());
			},
			div {
				class: "content",
				{props.children}
			}
		}
	}
}

#[component]
pub fn ContextMenuNestedContent(props: ContextMenuNestedProps) -> Element
{
	rsx! {
		{props.children}
	}
}

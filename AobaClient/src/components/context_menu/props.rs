use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuProps
{
	pub children: Element,
}

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuItemProps
{
	pub value: String,
	pub on_select: EventHandler<String>,
	pub children: Element,
}

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuTriggerProps
{
	pub children: Element,
}

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuContentProps
{
	pub children: Element,
}

#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuNestedProps
{
	pub children: Element,
}

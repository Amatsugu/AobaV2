use dioxus::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct DragContext
{
	pub is_dragging: Signal<bool>,
}

use dioxus::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct SelectionContext
{
	pub selected_items: Signal<Vec<String>>,
}

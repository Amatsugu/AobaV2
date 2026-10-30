use dioxus::prelude::*;

#[component]
pub fn BulkTagEditor(
	tags: Vec<Vec<String>>,
	on_add: EventHandler<String>,
	on_remove: EventHandler<String>,
) -> Element {
	rsx! {}
}

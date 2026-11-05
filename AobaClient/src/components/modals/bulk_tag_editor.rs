use dioxus::prelude::*;

use crate::{components::basic::flex::FlexColumn, rpc::aoba::BulkTagInfo};

#[component]
pub fn BulkTagEditor(
	tags: BulkTagInfo,
	on_add: EventHandler<String>,
	on_remove: EventHandler<String>,
) -> Element {
	rsx! {
		FlexColumn {
			for tag in tags.tags {
				div {
					{tag.tag} "{tag.count}"
				}
			}
		}
	}
}

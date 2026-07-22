use dioxus::prelude::*;
use dioxus_primitives::context_menu::ContextMenuItem;
use web_sys::window;

use crate::{
	components::MediaClassChangeEvent,
	contexts::SelectionContext,
	rpc::aoba::{MediaClass, MediaModel},
};

#[derive(Props, PartialEq, Clone)]
pub struct MediaItemContextMenuProps
{
	pub item: MediaModel,
	pub on_class_changed: EventHandler<MediaClassChangeEvent>,
	pub on_deleted: EventHandler<String>,
	pub bulk_change_class: EventHandler<MediaClass>,
}

#[component]
pub fn MediaItemContextMenuItems(props: MediaItemContextMenuProps) -> Element
{
	let item = props.item;
	let class = item.class();
	let id = item.id.unwrap().value;
	let download = item.media_url.clone();
	let selection_context: SelectionContext = use_context();
	let selection_count = selection_context.selected_items.len();
	rsx! {
		div{
			"This item"
		}
		ContextMenuItem {
			index: 0 as usize,
			value: id.clone(),
			on_select: move |id: String|{
				window().expect("Failed to get window")
					.location().set_href(&format!("/media/{}", id))
					.expect("Failed to open Url");
			},
			div{
				class: "contextItem",
				div{
					class: "label",
					"Details"
				}
			}
		},
		ContextMenuItem {
			index: 1 as usize,
			value: "{download}",
			on_select: move |url: String|{
				spawn(async move {
					window().expect("Failed to get window")
						.navigator()
						.clipboard()
						.write_text(&url).await
						.expect("Failed to copy url");
				});
			},
			div{
				class: "contextItem",
				div{
					class: "label",
					"Copy Url"
				}
			}
		},
		ContextMenuItem {
			index: 1 as usize,
			value: "{download}",
			on_select: move |url: String|{
				window().expect("Failed to get window").open_with_url_and_target(&url, "_blank").expect("Failed to open url");
			},
			div{
				class: "contextItem",
				div{
					class: "label",
					"Download"
				}
			}
		},
		if class != MediaClass::Standard {
			ContextMenuItem {
				index: 2 as usize,
				value: "{id}",
				on_select: move |id: String|{
					props.on_class_changed.call(MediaClassChangeEvent { id, class: MediaClass::Standard });
					// spawn(async move {
					// 	if set_class(&id, 0).await.is_ok(){
					// 		if let Some(handler) = props.on_class_changed{
					// 			handler.call(MediaClassChangeEvent { id, class: 0 });
					// 		}
					// 	}
					// });
				},
				div{
					class: "contextItem",
					div{
						class: "label",
						"Mark Standard"
					}
				}
			}
		}
		if class != MediaClass::Nsfw {
			ContextMenuItem {
				index: 3 as usize,
				value: "{id}",
				on_select: move |id: String|{
					props.on_class_changed.call(MediaClassChangeEvent { id, class: MediaClass::Nsfw });
					// spawn(async move {
					// 	if set_class(&id, 1).await.is_ok(){
					// 		if let Some(handler) = props.on_class_changed{
					// 			handler.call(MediaClassChangeEvent { id, class: 1 });
					// 		}
					// 	}
					// });
				},
				div{
					class: "contextItem",
					div{
						class: "label",
						"Mark NSFW"
					}
				}
			}
		}
		if class != MediaClass::Secret {
			ContextMenuItem {
				index: 4 as usize,
				value: "{id}",
				on_select: move |id: String|{
					props.on_class_changed.call(MediaClassChangeEvent { id, class: MediaClass::Secret });
					// spawn(async move {
					// 	if set_class(&id, 2).await.is_ok(){
					// 		if let Some(handler) = props.on_class_changed{
					// 			handler.call(MediaClassChangeEvent { id, class: 2 });
					// 		}
					// 	}
					// });
				},
				div{
					class: "contextItem",
					div{
						class: "label",
						"Mark Secret"
					}
				}
			}
		}
		ContextMenuItem {
			index: 5 as usize,
			value: "{id}",
			on_select: props.on_deleted,
			div{
				class: "contextItem",
				div{
					class: "label",
					"Delete"
				}
			}
		},
		if selection_count > 0{
			div{
				"{selection_count} Selected Items"
			}
			ContextMenuItem {
				index: 6 as usize,
				value: "{id}",
				on_select: move |_id|{
					props.bulk_change_class.call(MediaClass::Nsfw);
				},
				div{
					class: "contextItem",
					div{
						class: "label",
						"Mark as NSFW"
					}
				}
			}
			ContextMenuItem {
				index: 7 as usize,
				value: "{id}",
				on_select: move |_id|{
					props.bulk_change_class.call(MediaClass::Secret);
				},
				div{
					class: "contextItem",
					div{
						class: "label",
						"Mark as Secret"
					}
				}
			}
			ContextMenuItem {
				index: 8 as usize,
				value: "{id}",
				on_select: move |_id|{
					props.bulk_change_class.call(MediaClass::Standard);
				},
				div{
					class: "contextItem",
					div{
						class: "label",
						"Mark as Standard"
					}
				}
			}
		}
	}
}

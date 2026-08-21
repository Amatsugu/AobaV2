use std::time::Duration;

use dioxus::prelude::*;
use dioxus_primitives::context_menu::ContextMenuItem;
use web_sys::window;

use crate::{
	components::MediaClassChangeEvent,
	contexts::SelectionContext,
	models::toasts::{ToastCommand, ToastLevel, ToastsContext},
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
	let toasts_ctx = use_context::<ToastsContext>();
	let item = props.item;
	let class = item.class();
	let id = item.id.unwrap_or_default().value;
	let download = item.media_url.clone();
	let selection_context: SelectionContext = use_context();
	let selection_count = selection_context.selected_items.len();
	rsx! {
		div{
			"This item"
		}
		ContextMenuItem {
			index: 0_usize,
			value: id.clone(),
			on_select: move |id: String|{
				if window().and_then(|w| w.location().set_href(&format!("/media/{}", id)).ok()).is_none(){
					error!("Failed to open url");
				}
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
			index: 1_usize,
			value: download.clone(),
			on_select: move |url: String|{
				spawn(async move {
					if let Some(clipboard) = window().map(|w| w.navigator().clipboard()) && clipboard.write_text(&url).await.is_err() {
						error!("Failed to copy url");
						toasts_ctx.handle.send(ToastCommand::Push { title: "Failed to copy url".into(), message: None, level: ToastLevel::Error, duration: Some(Duration::from_secs(5)) });
					}else{
						toasts_ctx.handle.send(ToastCommand::Push { title: "Url copied".into(), message: None, level: ToastLevel::Info, duration: Some(Duration::from_secs(5)) });
					}
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
			index: 1_usize,
			value: download.clone(),
			on_select: move |url: String|{
				if window().and_then(|w| w.open_with_url_and_target(&url, "_blank").ok()).is_none(){
					error!("Failed to open download page");
				}
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
				index: 2_usize,
				value: id.clone(),
				on_select: move |id: String|{
					props.on_class_changed.call(MediaClassChangeEvent { id, class: MediaClass::Standard });
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
				index: 3_usize,
				value: id.clone(),
				on_select: move |id: String|{
					props.on_class_changed.call(MediaClassChangeEvent { id, class: MediaClass::Nsfw });
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
				index: 4_usize,
				value: id.clone(),
				on_select: move |id: String|{
					props.on_class_changed.call(MediaClassChangeEvent { id, class: MediaClass::Secret });
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
			index: 5_usize,
			value: id.clone(),
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
				index: 6_usize,
				value: id.clone(),
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
				index: 7_usize,
				value: id.clone(),
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
				index: 8_usize,
				value: id.clone(),
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

use std::time::Duration;

use dioxus::prelude::*;
use web_sys::window;

use crate::{
	components::{
		MediaClassChangeEvent, Modal, ModalInfo, OnItemSelectedEvent,
		basic::{
			Button, ButtonVariant,
			flex::{FlexColumn, FlexRow},
		},
		icons::{Check, DownloadFilled, EyeFilled, FireFilled, HiddenEyeFilled, Stack, TrashFilled},
	},
	models::toasts::{ToastCommand, ToastsContext},
	rpc::aoba::MediaClass,
};

#[component]
pub fn Toolbar(
	id: String,
	media_url: String,
	media_class: MediaClass,
	selected: bool,
	on_deleted: EventHandler<String>,
	on_class_change: EventHandler<MediaClassChangeEvent>,
	on_selected: EventHandler<OnItemSelectedEvent>,
) -> Element
{
	let toasts_ctx = use_context::<ToastsContext>();
	let mut toolbar_modal: Signal<Option<ModalInfo>> = use_signal(|| None);
	let tooltip: Signal<Option<String>> = use_signal(|| None);
	rsx! {
		div {
			class: "toolbar",
			MediaButton{
				arg: id.clone(),
				tooltip_text: {if selected { "Deselect" } else { "Select" }},
				tooltip,
				onclick: move |id: String|{
					on_selected.call((id.clone(), !selected));
				},
				Check { filled: selected }
			}
			if media_class != MediaClass::Standard{
				MediaButton{
					arg: id.clone(),
					tooltip_text: "Mark as Standard",
					tooltip,
					onclick: move |id: String|{
						on_class_change.call(MediaClassChangeEvent { id, class: MediaClass::Standard });
					},
					EyeFilled {}
				}
			}
			if media_class != MediaClass::Secret{
				MediaButton{
					arg: id.clone(),
					tooltip_text: "Mark as Secret",
					tooltip,
					onclick: move |id: String|{
						on_class_change.call(MediaClassChangeEvent { id, class: MediaClass::Secret });
					},
					HiddenEyeFilled {}
				}
			}
			if media_class != MediaClass::Nsfw{
				MediaButton{
					arg: id.clone(),
					tooltip_text: "Mark as NSFW",
					tooltip,
					onclick: move |id: String|{
						on_class_change.call(MediaClassChangeEvent { id, class: MediaClass::Nsfw });
					},
					FireFilled {}
				}
			}
			MediaButton{
				arg: media_url.clone(),
				tooltip_text: "Copy Url",
				tooltip,
				onclick: move |media_url: String|{
					spawn(async move {
						if let Some(clipboard) = window().map(|w| w.navigator().clipboard()) && clipboard.write_text(&media_url).await.is_err() {
							error!("Failed to copy url");
							toasts_ctx.push(ToastCommand::error("Failed to copy url").with_duration(Duration::from_secs(10)));
						}else{
							toasts_ctx.push(ToastCommand::info("Url copied").with_duration(Duration::from_secs(5)));
						}
					});
				},
				Stack { filled: true }
			}
			MediaButton{
				arg: media_url.clone(),
				tooltip_text: "Download",
				tooltip,
				onclick: move |url: String|{
					if window().and_then(|w| w.open_with_url_and_target(&url, "_blank").ok()).is_none(){
						toasts_ctx.push(ToastCommand::error("Failed to open download page"));
					}
				},
				DownloadFilled {}
			}
			MediaButton{
				arg: id.clone(),
				tooltip_text: "Delete",
				tooltip,
				onclick: move |id: String|{
					toolbar_modal.set(Some(ModalInfo{
						title: "Are you sure you want to delete this item?".into(),
						content: rsx!{
							DeleteModalContent {
								on_confirm: move |_|{
									on_deleted.call(id.clone());
								},
								on_close: move |_|{
									toolbar_modal.set(None);
								}
							}
						}
					}));
				},
				TrashFilled {}
			}
			if let Some(modal) = toolbar_modal.cloned(){
				Modal {
					is_open: true,
					title: modal.title,
					{modal.content}
				}
			}
		}
		if let Some(tooltip) = tooltip.cloned(){
			div{
				class: "toolbarTooltip",
				{tooltip}
			}
		}
	}
}

#[component]
fn DeleteModalContent(on_confirm: Callback, on_close: Callback) -> Element
{
	rsx! {
		FlexColumn {
			"Are you sure you want to delete these items? This cannot be undone!",
			FlexRow {
				Button{
					onclick: move |_| {
						on_close.call(());
						on_confirm.call(());
					},
					"Delete"
				}
				Button{
					variant: ButtonVariant::Cancel,
					onclick: move |_| {
						on_close.call(());
					},
					"Cancel"
				}
			}
		}
	}
}

#[component]
fn MediaButton(
	arg: Option<String>,
	onclick: Option<EventHandler<String>>,
	tooltip_text: String,
	tooltip: Signal<Option<String>>,
	children: Element,
) -> Element
{
	rsx! {
		div{
			class: "mediaButton",
			onclick: move |_| {
				let arg = arg.clone().unwrap_or_default();
				if let Some(handler) = onclick { handler.call(arg); }
			},
			onmouseover: move |_| tooltip.set(Some(tooltip_text.clone())),
			onmouseleave: move |_| tooltip.set(None),
			{children}
		}
	}
}

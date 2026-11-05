use std::time::Duration;

use crate::{
	HOST,
	components::{
		basic::flex::{FlexColumn, FlexRow},
		modals::BulkTagEditor,
	},
	models::toasts::{ToastCommand, ToastLevel, ToastsContext},
	rpc::{
		aoba::{BulkTagUpateRequest, Id, IdList},
		get_rpc_client,
	},
};
use dioxus::prelude::*;
use web_sys::window;

use crate::components::{
	Modal,
	basic::{Button, ButtonVariant},
};

const CLEAR_ICON: Asset = asset!("/assets/icons/clear.svg");
const TRASH_ICON: Asset = asset!("/assets/icons/trash.svg");
const TAG_ICON: Asset = asset!("/assets/icons/tag.svg");
const COPY_ICON: Asset = asset!("/assets/icons/copy-doc.svg");

#[derive(Debug, Clone)]
struct ModalInfo {
	pub title: String,
	pub content: Element,
}

#[component]
pub fn SelectionBar(
	selected_items: Signal<Vec<String>>,
	on_selection_cleared: EventHandler,
	on_items_delete: EventHandler,
) -> Element {
	let mut modal_content: Signal<Option<ModalInfo>> = use_signal(|| None);
	let toasts_ctx = use_context::<ToastsContext>();
	if selected_items.read().is_empty() {
		return rsx! {};
	} else {
		let selection_count = selected_items.read().len();
		return rsx! {
			div{
				id: "selectionBarContainer",
				div{
					id: "selectionBar",
					div{
						class: "info",
						"{selection_count} Items Selected"
					},
					div{
						class: "controls",
						BarButton{
							onclick: move |_| {
								spawn(async move {
									let links : Vec<String> = selected_items.read().iter()
										.map(|id| format!("{HOST}/m/{id}"))
										.collect();
									let joined = links.join("\n");
									match window().map(|w| w.navigator().clipboard()){
										Some(clipboard) => if clipboard.write_text(joined.as_str()).await.is_err(){
											error!("Failed to write to clipboard");
											toasts_ctx.handle.send(ToastCommand::Push { title: "Failed to write to clipboard".into(), message: None, level: ToastLevel::Error, duration: Some(Duration::from_secs(5)) });
										}else{
											toasts_ctx.handle.send(ToastCommand::Push { title: "Urls copied".into(), message: Some(format!("{} urls copied", selection_count)), level: ToastLevel::Info, duration: Some(Duration::from_secs(5)) });
										},
										None => toasts_ctx.handle.send(ToastCommand::Push { title: "Failed to write to clipboard".into(), message: Some("Unable to retreive clipboard".into()), level: ToastLevel::Error, duration: Some(Duration::from_secs(5)) }),
									}
								});
							},
							img{
								src: COPY_ICON
							}
						}
						BarButton{
							onclick: move |_|{
								modal_content.set(Some(ModalInfo {
									title: "Bulk Tags Editor".into(),
									content: rsx! {
										BulkTagEditModal {
											on_close: move |_| modal_content.set(None),
											selected_items
										}
									}
								}));
							},
							img{
								src: TAG_ICON
							}
						}
						BarButton{
							onclick: move|_|{
								modal_content.set(Some(ModalInfo {
									title: format!("Confirm Deletion of {} items", selection_count),
									content: rsx! {
										DeleteModalContent {
											on_close: move |_| modal_content.set(None),
											on_items_delete: on_items_delete
										}
									}
								 }));
							},
							img{
								src: TRASH_ICON
							}
						}
						BarButton{
							onclick: move |_|{
								on_selection_cleared.call(());
							},
							img{
								src: CLEAR_ICON
							}
						}
					}
				}
			}
			if let Some(modal) = modal_content.cloned() {
				Modal{
					title: modal.title,
					is_open: true,
					{modal.content}
				}
			}
		};
	}
}

#[component]
fn BulkTagEditModal(on_close: EventHandler, selected_items: Signal<Vec<String>>) -> Element {
	let toasts_ctx = use_context::<ToastsContext>();
	let tag_groups = use_resource(move || async move {
		let mut rpc = get_rpc_client();
		rpc.get_bulk_tag_info(IdList {
			value: selected_items
				.read()
				.iter()
				.map(|id| Id { value: id.clone() })
				.collect(),
		})
		.await
		.map(|r| r.into_inner())
	});

	rsx! {
		FlexColumn{
			match tag_groups() {
				Some(result) => match result {
					Ok(tags) => rsx! {
						BulkTagEditor {
							tags,
							on_add: move |tag| add_tag(selected_items, tag, toasts_ctx),
							on_remove: move |tag| remove_tag(selected_items, tag, toasts_ctx)
						 }
					},
					Err(status) => rsx! { "{status}" },
				},
				None => rsx! { "Loading..." },
			}
			FlexRow{
				Button{
					variant: ButtonVariant::Cancel,
					onclick: move |_| {
						on_close.call(());
					},
					"Cancel"
				}
				Button{
					variant: ButtonVariant::Accented,
					onclick: move |_| {
						on_close.call(());
					},
					"Save"
				}
			}
		}
	}
}

fn add_tag(items: Signal<Vec<String>>, tag: String, toasts_ctx: ToastsContext) {
	let item_count = items.read().len();
	spawn(async move {
		let mut rpc = get_rpc_client();
		let result = rpc
			.add_media_tags_bulk(BulkTagUpateRequest {
				media_ids: Some(IdList {
					value: items
						.read()
						.iter()
						.map(|id| Id { value: id.clone() })
						.collect(),
				}),
				tags: vec![tag.clone()],
			})
			.await
			.map(|r| r.into_inner());

		match result {
			Ok(_) => toasts_ctx.push(ToastCommand::Push {
				title: "Tag Added".into(),
				message: Some(format!("'{}' added to {} items", tag, item_count)),
				level: ToastLevel::Info,
				duration: Some(Duration::from_secs(5)),
			}),
			Err(status) => toasts_ctx.push(ToastCommand::Push {
				title: "Failed to add tag".into(),
				message: Some(status.to_string()),
				level: ToastLevel::Info,
				duration: Some(Duration::from_secs(5)),
			}),
		}
	});
}
fn remove_tag(items: Signal<Vec<String>>, tag: String, toasts_ctx: ToastsContext) {
	let item_count = items.read().len();
	spawn(async move {
		let mut rpc = get_rpc_client();
		let result = rpc
			.remove_media_tags_bulk(BulkTagUpateRequest {
				media_ids: Some(IdList {
					value: items
						.read()
						.iter()
						.map(|id| Id { value: id.clone() })
						.collect(),
				}),
				tags: vec![tag.clone()],
			})
			.await
			.map(|r| r.into_inner());

		match result {
			Ok(_) => toasts_ctx.push(ToastCommand::Push {
				title: "Tag Removed".into(),
				message: Some(format!("'{}' removed from {} items", tag, item_count)),
				level: ToastLevel::Info,
				duration: Some(Duration::from_secs(5)),
			}),
			Err(status) => toasts_ctx.push(ToastCommand::Push {
				title: "Failed to remove tag".into(),
				message: Some(status.to_string()),
				level: ToastLevel::Info,
				duration: Some(Duration::from_secs(5)),
			}),
		}
	});
}

#[component]
fn DeleteModalContent(on_close: EventHandler, on_items_delete: EventHandler) -> Element {
	rsx! {
		div{
			class: "vertFlexBox",
			"Are you sure you want to delete these items? This cannot be undone!",
			div{
				class: "horizFlexBox center",
				style: "gap:10px",
				Button{
					onclick: move |_| {
						on_close.call(());
						on_items_delete.call(());
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
fn BarButton(onclick: EventHandler<MouseEvent>, children: Element) -> Element {
	rsx! {
		div{
			class: "button",
			onclick:onclick,
			{children}
		}
	}
}

use dioxus::prelude::*;

use crate::{
	components::{
		ListItem, Modal,
		basic::{Button, Input, flex::FlexColumn},
	},
	models::toasts::{ToastCommand, ToastsContext},
	rpc::{
		aoba::{AccessToken, AccessTokenCreationRequest, AccessTokenFull},
		get_account_rpc_client,
	},
};

#[component]
pub fn AccessTokensManager() -> Element {
	let mut show_create = use_signal(|| false);
	let mut created_token: Signal<Option<AccessTokenFull>> = use_signal(|| None);
	rsx! {
		Button {
			onclick: move |_|{
				show_create.set(true);
			},
			"Create Token"
		}
		Modal{
			is_open: show_create.cloned(),
			title: "Create new token",
			CreateTokenForm {
				onsubmit: move |token| {
					created_token.set(Some(token));
					show_create.set(false);
				}
			}
		}
		if let Some(token) = created_token() {
			Modal{
				is_open: true,
				title: "New token",
				pre {
					"{token.token}"
				}
				Button {
					onclick: move |_| created_token.set(None),
					"Close"
				}
			}
		}
	}
}

#[component]
fn CreateTokenForm(onsubmit: EventHandler<AccessTokenFull>) -> Element {
	let label = use_signal(|| "".to_string());
	let toasts_ctx = use_context::<ToastsContext>();
	rsx! {
		FlexColumn{
			Input {
				label: "Label",
				name: "label",
				type: "text",
				value: label,
				required: true
			}
			Button{
				onclick: move |_|{
					spawn(async move {
						let mut rpc = get_account_rpc_client();
						let res = rpc.create_access_token(AccessTokenCreationRequest{
							label: label.cloned(),
							r#type: 1
						}).await.map_err(|s| s.message().to_string())
						.and_then(|r| r.into_inner().result.map(|r|Ok(r)).unwrap_or(Err("Unknown Error".to_string())))
						.and_then(|r| match r {
							crate::rpc::aoba::access_token_creation_response::Result::Token(token) => Ok(token),
							crate::rpc::aoba::access_token_creation_response::Result::Error(error) => Err(error.message),
						});

						match res {
							Ok(token) => {
								onsubmit.call(token);
								toasts_ctx.push(ToastCommand::info("Token Created").with_default_duration());
							},
							Err(err) => {
								toasts_ctx.push(ToastCommand::error_with_message("Failed to create Token", err).with_default_duration());
							},
						}

					});
				},
				"Create Token"
			}
		}
	}
}

#[component]
fn TokenEntry(token: AccessToken) -> Element {
	rsx! {
		ListItem {
			title: token.label,
			"Last Used: {token.last_used}"
		}
	}
}

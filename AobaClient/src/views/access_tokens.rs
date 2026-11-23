use dioxus::prelude::*;

use crate::{
	components::{
		ListItem, Modal, Notif, NotifType,
		basic::{Button, ButtonVariant, Input, flex::FlexColumn},
	},
	models::toasts::{ToastCommand, ToastsContext},
	rpc::{
		aoba::{AccessToken, AccessTokenCreationRequest, AccessTokenFull, AccessTokenList, AccessTokenType},
		get_account_rpc_client,
	},
};

#[component]
pub fn AccessTokensManager() -> Element
{
	let mut show_create = use_signal(|| false);
	let mut created_token: Signal<Option<AccessTokenFull>> = use_signal(|| None);
	let mut token_list = use_token_list();
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
				h3 { "Token" }
				pre {
					class: "codeSelect",
					{token.token}
				}
				if let Some(dst) = token.sharex_destination{
					h3 { "ShareX" }
					pre {
						class: "codeSelect",
						{dst}
					}
				}
				Button {
					onclick: move |_| {
						created_token.set(None);
						token_list.restart();
					},
					"Close"
				}
			}
		}
		match token_list(){
			Some(tokens) => rsx! { TokenList { tokens, onrevoke: move||{
				token_list.restart();
			} } },
			None => rsx!{"Loading Tokens..."},
		}
	}
}

#[component]
fn CreateTokenForm(onsubmit: EventHandler<AccessTokenFull>) -> Element
{
	let label = use_signal(|| "".to_string());
	let mut token_type = use_signal(|| AccessTokenType::Metrics);
	let toasts_ctx = use_context::<ToastsContext>();
	rsx! {
		FlexColumn{
			gap: "10px",
			Input {
				label: "Label",
				name: "label",
				type: "text",
				value: label,
				required: true
			}
			label{
				"Type"
				select {
					name: "type",
					required: true,
					oninput: move |e| match e.value().as_str() {
						"1" => token_type.set(AccessTokenType::Metrics),
						"2" => token_type.set(AccessTokenType::Upload),
						_ => token_type.set(AccessTokenType::Unspecified),
					},
					option {
						value: "1",
						"Metrics"
					}
					option {
						value: "2",
						"Upload"
					}
				}
			}
			Button{
				onclick: move |_|{
					spawn(async move {
						let mut rpc = get_account_rpc_client();
						let res = rpc.create_access_token(AccessTokenCreationRequest{
							label: label.cloned(),
							r#type: token_type.cloned().into(),
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
fn TokenList(tokens: Result<AccessTokenList, String>, onrevoke: Callback) -> Element
{
	rsx! {
		match tokens.map(|t|t.tokens) {
			Ok(tokens) => rsx! {
				for token in tokens {
					TokenEntry { token, onrevoke: onrevoke }
				}
			},
		Err(err) => rsx! { Notif { message: err, type: NotifType::Error } },
		}
	}
}

#[component]
fn TokenEntry(token: AccessToken, onrevoke: Callback) -> Element
{
	let toasts_ctx = use_context::<ToastsContext>();
	let id = token.token_id.unwrap_or_default();
	rsx! {
		ListItem {
			title: token.label,
			controls: rsx!{
				Button{
					onclick: move |_|{
						let id = id.clone();
						spawn(async move {
							let mut rpc = get_account_rpc_client();
							match rpc.revoke_access_token(id).await {
								Ok(_) => {
									onrevoke.call(());
									toasts_ctx.push(ToastCommand::info("Token Revoked").with_default_duration());
								},
								Err(err) => toasts_ctx.push(
									ToastCommand::error_with_message("Failed to Revoke", err.message())
										.with_default_duration()
								),
							}
						});
					},
					variant: ButtonVariant::Cancel,
					"Revoke"
				}
			},
			"Last Used: {token.last_used}"
		}
	}
}

fn use_token_list() -> Resource<Result<AccessTokenList, String>>
{
	use_resource(async || {
		let mut rpc = get_account_rpc_client();
		rpc.get_access_tokens(())
			.await
			.map(|r| r.into_inner())
			.map_err(|s| s.message().to_string())
	})
}

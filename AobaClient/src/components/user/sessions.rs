use dioxus::prelude::*;

use crate::{
	components::{
		ListItem, Notif, NotifType,
		basic::{Button, ButtonVariant, flex::FlexColumn},
	},
	models::toasts::{ToastCommand, ToastsContext},
	rpc::{
		aoba::{Id, UserSession, UserSessionList},
		get_account_rpc_client,
	},
};

#[component]
pub fn UserSessions() -> Element
{
	let toasts_ctx = use_context::<ToastsContext>();
	let mut sessions = use_resource(async move || {
		let mut rpc = get_account_rpc_client();
		let sessions = rpc.get_user_sessions(()).await.map_err(|e| e.to_string())?;
		sessions
			.into_inner()
			.result
			.map(|r| match r
			{
				crate::rpc::aoba::user_session_list_response::Result::List(user_session_list) => Ok(user_session_list),
				crate::rpc::aoba::user_session_list_response::Result::Error(error) => Err(error.message),
			})
			.unwrap_or_else(|| Err("Failed to load sessions".to_string()))
	});
	rsx! {
		match sessions() {
			Some(res) => match res {
				Ok(list) => rsx! { List {
					list,
					on_remove: move |id| {
						spawn(async move {
							let mut rpc = get_account_rpc_client();
							let res = rpc.revoke_session(Id {value: id}).await;
							match res {
								Ok(_) => {
									toasts_ctx.push(ToastCommand::info("Session Revoked").with_default_duration());
									sessions.restart();
								},
								Err(err) => toasts_ctx.push(ToastCommand::error_with_message("Failed to Revoke", err.code())
									.with_default_duration()),
							}
						});
					}
				}},
				Err(err) => rsx!{
					Notif{
						message: err,
						type: NotifType::Error
					}
				},
			},
			None => rsx! { "Loading..." },
		}
	}
}

#[component]
fn List(list: UserSessionList, on_remove: EventHandler<String>) -> Element
{
	rsx! {
		h3 { "Active Sessions" }
		FlexColumn {
			gap: "15px",
			for session in list.sessions {
				Session {
					session,
					on_remove
				}
			}
		}
	}
}

#[component]
fn Session(session: UserSession, on_remove: EventHandler<String>) -> Element
{
	let cur_session_label = if session.is_current { "[Current Session]" } else { "" };
	let id = session.token_id.unwrap_or_default().value;
	rsx! {
		ListItem{
			title: "{session.label} {cur_session_label}",
			controls: rsx!{
				Button{
					variant: ButtonVariant::Cancel,
					onclick: move |_|
					{
						on_remove.call(id.clone());
					},
					"Revoke"
				}
			},
			div{"Created: {session.date_created}"}
			div{"Last Used: {session.last_used}"}
		}
	}
}

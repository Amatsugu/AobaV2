use dioxus::prelude::*;
use tonic::IntoRequest;

use crate::{
	components::{basic::Input, Notif, NotifType},
	contexts::AuthContext,
	rpc::{aoba::Credentials, get_auth_rpc_client},
};

#[component]
pub fn Login() -> Element {
	let username = use_signal(|| "".to_string());
	let password = use_signal(|| "".to_string());
	let mut error: Signal<Option<String>> = use_signal(|| None);
	let mut auth_context = use_context::<AuthContext>();

	let login = move |e: Event<MouseData>| {
		e.prevent_default();
		if username.cloned().is_empty() || password.cloned().is_empty() {
			error.set(Some("Username and Password are required".into()));
			return;
		}

		spawn(async move {
			let mut auth = get_auth_rpc_client();
			let result = auth
				.login(
					Credentials {
						user: username.cloned(),
						password: password.cloned(),
					}
					.into_request(),
				)
				.await;
			match result {
				Ok(res) => {
					match res.into_inner().result.unwrap() {
						crate::rpc::aoba::login_response::Result::Jwt(jwt) => {
							auth_context.login(jwt.token);
						}
						crate::rpc::aoba::login_response::Result::Error(login_error) => {
							auth_context.logout();
							error.set(Some(login_error.message));
						}
					};
				}
				Err(_err) => {
					auth_context.logout();
				}
			}
		});
	};

	rsx! {
		div { id: "centralModal",
			if let Some(err) = error.cloned() {
				Notif { r#type: NotifType::Error, message: err }
			}
			form {
				Input {
					r#type: "text",
					name: "username",
					label: "Username",
					value: username,
					required: true,
				}
				Input {
					r#type: "password",
					name: "password",
					label: "Password",
					value: password,
					required: true,
				}
				button { onclick: login, "Login!" }
			}
		}
	}
}

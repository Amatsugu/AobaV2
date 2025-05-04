use dioxus::prelude::*;
use tonic::IntoRequest;

use crate::{
	components::basic::{Button, Input},
	contexts::AuthContext,
	rpc::{aoba::Credentials, get_auth_rpc_client},
};

#[component]
pub fn Login() -> Element {
	let username = use_signal(|| "".to_string());
	let password = use_signal(|| "".to_string());
	let mut auth_context = use_context::<AuthContext>();

	let onclick = move |_| {
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
							auth_context.jwt.set(Some(jwt.token));
						}
						crate::rpc::aoba::login_response::Result::Error(_login_error) => {
							auth_context.jwt.set(None);
						}
					};
				}
				Err(_err) => {
					auth_context.jwt.set(None);
				}
			}
		});
	};

	rsx! {
		div{
			id: "centralModal",
			form{
				Input { type : "text", name: "username", label: "Username", value: username, required: true },
				Input { type : "password", name: "password", label: "Password", value: password, required: true },
				Button {
					text: "Login!",
					onclick: onclick
				}
			}
		}
	}
}

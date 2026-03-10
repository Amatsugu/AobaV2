use dioxus::prelude::*;

use crate::components::basic::Button;

#[component]
pub fn PasskeyRegistrationButton() -> Element
{
	rsx! {
		Button{
			text: "Register Passkey",
			onclick: move |e| {

			}
		}
	}
}

fn start_passkey_registration() {}

fn create_credential() {}

#[component]
pub fn PasskeyLoginButton() -> Element
{
	rsx! {
		Button{
			text: "Login with Passkey"
		}
	}
}

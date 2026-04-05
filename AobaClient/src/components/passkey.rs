use dioxus::prelude::*;
use web_sys::{CredentialCreationOptions, window};

use crate::components::basic::Button;

#[component]
pub fn PasskeyRegistrationButton() -> Element {
	rsx! {
		Button{
			text: "Register Passkey",
			onclick: move |_| {
				start_passkey_registration();
			}
		}
	}
}

fn start_passkey_registration() {
	create_credential();
}

fn create_credential() {
	let credentials = window()
		.expect("Failed to get window")
		.navigator()
		.credentials();

	let opts = CredentialCreationOptions::new();
	let _result = credentials.create_with_options(&opts);
}

#[component]
pub fn PasskeyLoginButton() -> Element {
	rsx! {
		Button{
			text: "Login with Passkey"
		}
	}
}

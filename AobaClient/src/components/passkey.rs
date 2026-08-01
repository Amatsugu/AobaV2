use dioxus::prelude::*;
use js_sys::{Uint8Array, wasm_bindgen::JsValue};
use web_sys::{
	CredentialCreationOptions, PublicKeyCredentialCreationOptions, PublicKeyCredentialRpEntity,
	PublicKeyCredentialUserEntity, window,
};

use crate::{components::basic::Button, rpc::aoba::PasskeyCredentialCreateOptions};

#[component]
pub fn PasskeyRegistrationButton() -> Element
{
	rsx! {
		Button{
			text: "Register Passkey",
			onclick: move |_| {
				start_passkey_registration();
			}
		}
	}
}

fn start_passkey_registration()
{
	// create_credential(todo!());
}

#[allow(dead_code)]
fn create_credential(req_opts: PasskeyCredentialCreateOptions)
{
	if let Some(credentials) = window().map(|w| w.navigator().credentials())
		&& let Some(opts) = opts_from_rpc(req_opts)
	{
		let _result = credentials.create_with_options(&opts);
		todo!()
	}
}

#[allow(dead_code)]
fn opts_from_rpc(rpc_opts: PasskeyCredentialCreateOptions) -> Option<CredentialCreationOptions>
{
	if let Some(opt_user) = &rpc_opts.user
		&& let Some(opt_rp) = &rpc_opts.rp
	{
		let opts = CredentialCreationOptions::new();
		let rp = PublicKeyCredentialRpEntity::new(&opt_rp.name);
		rp.set_id(&opt_rp.id);

		let user = PublicKeyCredentialUserEntity::new_with_u8_array(
			&opt_user.name,
			&opt_user.display_name,
			&to_u8_array(&opt_user.id),
		);
		let pub_key_opts = PublicKeyCredentialCreationOptions::new_with_u8_array(
			&to_u8_array(&rpc_opts.challenge),
			&JsValue::undefined(),
			&rp,
			&user,
		);
		opts.set_public_key(&pub_key_opts);

		Some(opts)
	}
	else
	{
		None
	}
}

#[allow(dead_code)]
fn to_u8_array(_value: &String) -> Uint8Array
{
	todo!()
}

#[component]
pub fn PasskeyLoginButton() -> Element
{
	rsx! {
		Button{
			text: "Login with Passkey"
		}
	}
}

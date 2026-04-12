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
	create_credential(todo!());
}

fn create_credential(req_opts: PasskeyCredentialCreateOptions)
{
	let window = window().expect("Window does not exist");
	let credentaials = window.navigator().credentials();

	let opts = opts_from_rpc(req_opts);

	let result = credentaials.create_with_options(&opts);
	todo!()
}

fn opts_from_rpc(rpc_opts: PasskeyCredentialCreateOptions) -> CredentialCreationOptions
{
	let opt_user = &rpc_opts.user.expect("user is missing");
	let opt_rp = &rpc_opts.rp.expect("rp is missing");
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
	//pub_key_opts.set_exclude_credentials(val);
	opts.set_public_key(&pub_key_opts);

	return opts;
}

fn to_u8_array(value: &String) -> Uint8Array
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

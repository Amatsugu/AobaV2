use dioxus::prelude::*;

use crate::{
	components::{ListItem, basic::Button},
	rpc::{aoba::StoredPasskeyCredential, get_account_rpc_client},
};

#[component]
pub fn PasskeyManager() -> Element {
	let creds = use_resource(async || {
		let mut rpc = get_account_rpc_client();
		rpc.get_stored_passkey_credentials(())
			.await
			.map_err(|e| e.to_string())?
			.into_inner()
			.result
			.unwrap_or_else(|| Err("".to_string()))
	});
	rsx! {}
}

#[component]
fn PasskeyList(creds: Vec<StoredPasskeyCredential>) -> Element {
	rsx! {
		for cred in creds {
			PasskeyEntry { cred }
		}
	}
}

#[component]
fn PasskeyEntry(cred: StoredPasskeyCredential) -> Element {
	rsx! {
		ListItem{
			title: "Passkey",
			controls: rsx!{
				Button { "Rename" }
				Button { "Delete" }
			},
			"Lastused: -"
		}
	}
}

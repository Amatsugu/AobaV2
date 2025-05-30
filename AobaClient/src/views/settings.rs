use dioxus::prelude::*;

use crate::rpc::get_rpc_client;

#[component]
pub fn Settings() -> Element {
	let dst = use_resource(async move || {
		let result = get_rpc_client().get_share_x_destination(()).await;
		if let Ok(d) = result {
			if let Some(r) = d.into_inner().dst_result {
				return match r {
					crate::rpc::aoba::share_x_response::DstResult::Destination(json) => json,
					crate::rpc::aoba::share_x_response::DstResult::Error(err) => err,
				};
			}
			return "No Result".to_string();
		}
		let err = result.err().unwrap();
		let status = err.message();
		return format!("Failed to load config: {status}").to_string();
	});

	let d = dst.cloned().unwrap_or("".to_string());

	rsx! {
		"this is settings"
		div {
			pre { class: "codeSelect", "{d}" }
		}
	}
}

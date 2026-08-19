use dioxus::prelude::*;

use crate::{
	components::{MetricsToken, PasskeyRegistrationButton},
	rpc::get_rpc_client,
};

#[component]
pub fn Settings() -> Element {
	let dst = use_resource(async move || {
		let result = get_rpc_client().get_share_x_destination(()).await;
		match result {
			Ok(d) => {
				if let Some(r) = d.into_inner().dst_result {
					use crate::rpc::aoba::share_x_response::DstResult;
					return match r {
						DstResult::Destination(json) => json,
						DstResult::Error(err) => err,
					};
				}
				"No Result".to_string()
			}
			Err(err) => {
				let status = err.message();
				format!("Failed to load config: {status}").to_string()
			}
		}
	});

	let d = dst.cloned().unwrap_or("Loading...".to_string());

	rsx! {
		h3 { "ShareX Config" }
		div {
			pre { class: "codeSelect", {d} }
		}
		MetricsToken {  }
		PasskeyRegistrationButton { }
	}
}

use dioxus::prelude::*;

use crate::{
	components::{MetricsToken, PasskeyRegistrationButton},
	rpc::get_rpc_client,
};

#[component]
pub fn Settings() -> Element
{
	let dst = use_resource(async move || {
		let result = get_rpc_client().get_share_x_destination(()).await;
		match result
		{
			Ok(d) =>
			{
				if let Some(r) = d.into_inner().dst_result
				{
					return match r
					{
						crate::rpc::aoba::share_x_response::DstResult::Destination(json) => json,
						crate::rpc::aoba::share_x_response::DstResult::Error(err) => err,
					};
				}
				"No Result".to_string()
			}
			Err(err) =>
			{
				let status = err.message();
				format!("Failed to load config: {status}").to_string()
			}
		}
	});

	let d = dst.cloned().unwrap_or_default();

	rsx! {
		"this is settings"
		div {
			pre { class: "codeSelect", {d} }
		}
		MetricsToken {  }
		PasskeyRegistrationButton { }
	}
}

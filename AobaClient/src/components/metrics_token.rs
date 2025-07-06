use dioxus::prelude::*;

use crate::rpc::get_metrics_rpc_client;

#[component]
pub fn MetricsToken() -> Element {
	let token = use_resource(async move || {
		let response = get_metrics_rpc_client().get_token(()).await;

		if let Ok(d) = response {
			let jwt = d.into_inner();
			return jwt.token;
		}
		return "".to_string();
	});

	let token_value = token.cloned().unwrap_or("".to_string());

	return rsx! {
		pre {
			class: "codeSelect",
			"{token_value}"
		}
	};
}

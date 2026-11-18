use dioxus::prelude::*;

use crate::{
	components::{PasskeyRegistrationButton, basic::Button, user::UserSessions},
	route::Route,
};

#[component]
pub fn Settings() -> Element {
	rsx! {
		ManageTokens {  }
		PasskeyRegistrationButton {}
		UserSessions {}
	}
}

#[component]
fn ManageTokens() -> Element {
	rsx! {
		h3 { "Tokens" }
		Link {
			to: Route::AccessTokensManager {  },
			Button { "Manage Tokens" }
		 }
	}
}

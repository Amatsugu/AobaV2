use dioxus::prelude::*;

use crate::components::{MetricsToken, PasskeyRegistrationButton, sharex::ShareXToken, user::UserSessions};

#[component]
pub fn Settings() -> Element
{
	rsx! {
		ShareXToken {}
		MetricsToken {}
		PasskeyRegistrationButton {}
		UserSessions {}
	}
}

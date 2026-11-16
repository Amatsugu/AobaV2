use dioxus::prelude::*;

use crate::{
	Route,
	components::{Navbar, toasts::ToastsDisplay},
	contexts::AuthContext,
	rpc::use_unauth_hook,
	views::Login,
};

#[component]
pub fn MainLayout() -> Element
{
	let auth_context = use_context::<AuthContext>();

	use_unauth_hook(auth_context);

	if !auth_context.logged_in.cloned()
	{
		return rsx! {
			Login { }
		};
	}
	return rsx! {
		ToastsDisplay {}
		Navbar { }
		div {
			id: "content",
			Outlet::<Route> { }
		}
	};
}

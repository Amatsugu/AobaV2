use dioxus::prelude::*;

use crate::{Route, components::Navbar, contexts::AuthContext, views::Login};

#[component]
pub fn MainLayout() -> Element {
	let auth_context = use_context::<AuthContext>();

	if auth_context.jwt.cloned().is_none() {
		return rsx! {
			Login {}
		};
	}

	return rsx! {
		Navbar {}
		div { id: "content", Outlet::<Route> {} }
	};
}

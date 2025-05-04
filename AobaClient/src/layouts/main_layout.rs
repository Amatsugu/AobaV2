use dioxus::prelude::*;

use crate::{components::Navbar, contexts::AuthContext, layouts::BasicLayout, views::Login, Route};

#[component]
pub fn MainLayout() -> Element {
	let auth_context = use_context::<AuthContext>();

	// if auth_context.jwt.cloned().is_none() {
	// 	return rsx! { Login {  } };
	// }

	return rsx! {
		Navbar {}
		Outlet::<Route> {}
	};
}

use dioxus::prelude::*;

use crate::{
	Route,
	components::{ContextMenuRenderer, ContextMenuRoot, Navbar},
	contexts::AuthContext,
	views::Login,
};

#[component]
pub fn MainLayout() -> Element {
	let auth_context = use_context::<AuthContext>();

	if auth_context.jwt.cloned().is_none() {
		return rsx! {
			Login { }
		};
	}

	let mut ct_renderer = use_context::<ContextMenuRenderer>();

	return rsx! {
		ContextMenuRoot {  }
		Navbar { }
		div {
			id: "content",
			onclick: move |_| {
				ct_renderer.close();
			},
			oncontextmenu: move |_| {
				ct_renderer.close();
			},
			Outlet::<Route> { }
		}
	};
}

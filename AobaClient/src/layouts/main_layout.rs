use dioxus::prelude::*;

use crate::{components::Navbar, Route};

#[component]
pub fn MainLayout() -> Element {
	rsx! {
		Navbar {}
		Outlet::<Route> {}
	}
}

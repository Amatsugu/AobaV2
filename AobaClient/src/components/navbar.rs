use dioxus::prelude::*;

use crate::{Route, contexts::AuthContext};

const NAV_CSS: Asset = asset!("/assets/style/nav.scss");
const NAV_ICON: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn Navbar() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: NAV_CSS }
		nav {
			Branding {}
			MainNaviagation {}
			Widgets {}
			Utils {}
		}
	}
}

#[component]
pub fn MainNaviagation() -> Element {
	rsx! {
		div { class: "mainNav",
			Link { class: "navItem", to: Route::Home {}, "Home" }
			Link { class: "navItem", to: Route::Settings {}, "Settings" }
		}
	}
}

#[component]
pub fn Branding() -> Element {
	rsx! {
		div { class: "branding",
			img { src: NAV_ICON, alt: "Aoba" }
		}
	}
}

#[component]
pub fn Widgets() -> Element {
	rsx! {
		div { class: "widgets" }
	}
}

#[component]
pub fn Utils() -> Element {
	let mut auth_context = use_context::<AuthContext>();

	rsx! {
		div { class: "utils",
			div { onclick: move |_| auth_context.logout(), "Logout" }
		}
	}
}

use dioxus::prelude::*;

use crate::Route;

const NAV_CSS: Asset = asset!("/assets/style/nav.scss");

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
		div { class: "branding" }
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
	rsx! {
		div { class: "utils" }
	}
}

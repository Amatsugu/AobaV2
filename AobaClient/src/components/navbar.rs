use dioxus::prelude::*;

use crate::{
	Route,
	components::icons::{Cog, Image, LockFilled},
	contexts::AuthContext,
	env::APP_VERSION,
};

const NAV_CSS: Asset = asset!("/assets/style/nav.scss");
const NAV_ICON: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn Navbar() -> Element
{
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
pub fn MainNaviagation() -> Element
{
	rsx! {
		div { class: "mainNav",
			Link {
				class: "navItem",
				to: Route::Home { page: None, q: None },
				Image {}
			}
			Link { class: "navItem", to: Route::Settings {}, Cog{} }
		}
	}
}

#[component]
pub fn Branding() -> Element
{
	rsx! {
		div { class: "branding",
			img { height: "40", width: "40", src: NAV_ICON, alt: "Aoba" }
		}
	}
}

#[component]
pub fn Widgets() -> Element
{
	rsx! {
		div { class: "widgets" }
	}
}

#[component]
pub fn Utils() -> Element
{
	let mut auth_context = use_context::<AuthContext>();
	let version = APP_VERSION;
	rsx! {
		div { class: "utils",
			div { class: "vertical", {version} }
			div { class: "navItem", onclick: move |_| auth_context.logout(), LockFilled {} }
		}
	}
}

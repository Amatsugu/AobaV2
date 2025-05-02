pub mod components;
mod layouts;
pub mod models;
pub mod route;
pub mod views;

use dioxus::prelude::*;
use route::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/style/main.scss");

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	rsx! {
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
		document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
		document::Link { rel: "stylesheet", href: MAIN_CSS }
		document::Link {
			rel: "stylesheet",
			href: "https://fonts.googleapis.com/css2?family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap",
		}
		Router::<Route> {}
	}
}

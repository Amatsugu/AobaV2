pub mod components;
pub mod contexts;
mod layouts;
pub mod models;
pub mod route;
pub mod rpc;
pub mod views;

use contexts::AuthContext;
use dioxus::prelude::*;
use route::Route;

#[cfg(debug_assertions)]
pub const HOST: &'static str = "http://localhost:8081";
#[cfg(debug_assertions)]
pub const RPC_HOST: &'static str = "http://localhost:8081";
#[cfg(not(debug_assertions))]
pub const RPC_HOST: &'static str = "https://grpc.aoba.app:8443";
#[cfg(not(debug_assertions))]
pub const HOST: &'static str = "https://aoba.app";

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/style/main.scss");
const INPUT_CSS: Asset = asset!("/assets/style/inputs.scss");

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	let _auth_state = use_context_provider(|| AuthContext::new());
	rsx! {
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
		document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
		document::Link { rel: "stylesheet", href: MAIN_CSS }
		document::Link { rel: "stylesheet", href: INPUT_CSS }
		document::Link {
			rel: "stylesheet",
			href: "https://fonts.googleapis.com/css2?family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap",
		}
		Router::<Route> { }
	}
}

use dioxus::prelude::*;

use crate::route::Route;

#[component]
pub fn BasicLayout() -> Element {
	rsx! {
		Outlet::<Route> {}
	}
}

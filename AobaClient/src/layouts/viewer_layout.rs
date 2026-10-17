use dioxus::prelude::*;

use crate::components::{
	icons::{LeftChevron, RightChevron},
	toasts::ToastsDisplay,
};
const CSS: Asset = asset!("/assets/style/collections.scss");

#[component]
pub fn ViewerLayout() -> Element {
	rsx! {
		document::Link{ href: CSS, rel: "stylesheet" }
		ToastsDisplay{}
		div {
			id: "viewerContent",
			TopBar {  }
			DisplayArea {  }
			Sidebar {  }
			MediaList {}
		}
	}
}

#[component]
fn TopBar() -> Element {
	rsx! {
		div {
			id: "titleBar",
			"Collection Name"
		}
	}
}

#[component]
fn DisplayArea() -> Element {
	rsx! {
		div {
			id: "displayArea"
		}
	}
}

#[component]
fn MediaList() -> Element {
	rsx! {
		div {
			id: "mediaList",
			div {
				class: "prev",
				LeftChevron {}
			}
			div {
				class: "list",
				for _ in 0..40 {
					CollectionItem {  }
				}
			}
			div {
				class: "next",
				RightChevron {}
			}
		}
	}
}

#[component]
fn CollectionItem() -> Element {
	rsx! {
		div {
			class: "collectionItem"
		}
	}
}

#[component]
fn Sidebar() -> Element {
	rsx! {
		div {
			id: "sidebar"

		}
	}
}

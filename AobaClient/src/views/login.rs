use dioxus::prelude::*;

use crate::components::basic::{Button, Input};

#[component]
pub fn Login() -> Element {
	rsx! {
		div{
			id: "centralModal",
			form{
				Input { type : "text", name: "username", label: "Username" },
				Input{ type : "password", name: "password", label: "Password" },
				Button{text: "Login!"}
			}
		}
	}
}

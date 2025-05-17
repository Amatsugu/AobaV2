use dioxus::{html::u::icon, prelude::*};

use crate::components::icons;

#[derive(PartialEq, Clone, Props)]
pub struct NotifProps {
	r#type: Option<NotifType>,
	message: String,
}

#[derive(PartialEq, Clone)]
pub enum NotifType {
	Notice,
	Error,
	Warning,
}

#[component]
pub fn Notif(props: NotifProps) -> Element {
	let t = props.r#type.unwrap_or(NotifType::Notice);
	let type_class = match t {
		NotifType::Notice => "notice",
		NotifType::Error => "error",
		NotifType::Warning => "warning",
	};
	let m = props.message;
	rsx! {
		div{
			class: "notif {type_class}",
			div{
				class: "icon",
				match t {
					NotifType::Notice => icons::Error(),
					NotifType::Error => icons::Error(),
					NotifType::Warning => icons::Warn(),
				}
			}
			div{
				class: "message",
				"{m}"
			}
		}
	}
}

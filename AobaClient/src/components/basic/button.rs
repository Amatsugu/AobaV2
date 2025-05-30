use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
	pub variant: Option<ButtonVariant>,
	pub text: String,
	pub onclick: Option<EventHandler<Event<MouseData>>>,
}

#[derive(PartialEq, Clone)]
pub enum ButtonVariant {
	Base,
	Muted,
	Accented,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
	rsx! {
		button {
			onclick: move |event| {
			    event.prevent_default();
			    if let Some(h) = props.onclick {
			        h.call(event);
			    }
			},
			"{props.text}"
		}
	}
}

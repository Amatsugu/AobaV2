use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
	variant: Option<ButtonVariant>,
	text: String,
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
		button { "{props.text}" }
	}
}

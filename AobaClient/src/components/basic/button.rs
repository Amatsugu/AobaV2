use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps
{
	pub variant: Option<ButtonVariant>,
	pub text: String,
	pub onclick: Option<EventHandler<Event<MouseData>>>,
}

#[derive(PartialEq, Clone, Default)]
pub enum ButtonVariant
{
	#[default]
	Base,
	Muted,
	Accented,
	Cancel,
}

#[component]
pub fn Button(props: ButtonProps) -> Element
{
	let variantClass = match props.variant.unwrap_or_default()
	{
		ButtonVariant::Base => "",
		ButtonVariant::Muted => "muted",
		ButtonVariant::Accented => "accented",
		ButtonVariant::Cancel => "cancel",
	};
	rsx! {
		button {
			class: "{variantClass}",
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

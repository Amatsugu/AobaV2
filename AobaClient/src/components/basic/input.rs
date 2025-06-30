use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct InputProps {
	pub r#type: Option<String>,
	pub value: Option<Signal<String>>,
	pub label: Option<String>,
	pub placeholder: Option<String>,
	pub name: String,
	pub oninput: Option<EventHandler<FormEvent>>,
	pub required: Option<bool>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
	let label = props.label.unwrap_or("".into());
	let ph = props.placeholder.unwrap_or(label.clone());
	rsx! {
		label {
			"{label}"
			input {
				r#type: props.r#type.unwrap_or("text".into()),
				value: props.value,
				oninput: move |e| {
				    if let Some(mut s) = props.value {
				        s.set(e.value());
				    }
				},
				name: props.name,
				placeholder: ph,
				required: props.required,
			}
		}
	}
}

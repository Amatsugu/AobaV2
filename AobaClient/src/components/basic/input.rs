use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct InputProps {
	pub r#type: Option<String>,
	pub value: Option<String>,
	pub label: Option<String>,
	pub placeholder: Option<String>,
	pub name: String,
}

#[component]
pub fn Input(props: InputProps) -> Element {
	let label = props.label.unwrap_or("".into());
	let ph = props.placeholder.unwrap_or(label.clone());
	rsx! {
		label {
			"{label}",
			input {
				type : props.r#type.unwrap_or("text".into()),
				value: props.value,
				name: props.name,
				placeholder:ph
			}
		}
	}
}

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ListItemProps
{
	controls: Option<Element>,
	title: String,
	children: Element,
}

#[component]
pub fn ListItem(props: ListItemProps) -> Element
{
	rsx! {
		div {
			class: "listItem",
			div {
				class: "head",
				div{
					class: "title",
					"{props.title}"
				}
				div{
					class: "controls",
					{props.controls}
				}
			}
			div{
				class: "content",
				{props.children}
			}
		 }
	}
}

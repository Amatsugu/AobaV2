use dioxus::prelude::*;

#[component]
pub fn Modal(title: String, is_open: Option<bool>, children: Element) -> Element
{
	if !is_open.unwrap_or(false)
	{
		return rsx! {};
	}
	return rsx! {
		div{
			class: "modalContainer",
			ModalWindow { title, {children} }
		}
	};
}

#[component]
fn ModalWindow(title: String, children: Element) -> Element
{
	rsx! {
		div{
			class: "modal",
			div{
				class: "titleBar",
				div{
					class: "title",
					{title}
				}
				div{
					class: "controls",

				}
			}
			div{
				class: "content",
				{children}
			}
		}
	}
}

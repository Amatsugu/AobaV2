use dioxus::prelude::*;

#[component]
pub fn FlexRow(children: Element) -> Element {
	rsx! {
		div{
			style: "display: flex; flex-direction:row",
			{children}
		}
	}
}

#[component]
pub fn FlexColumn(children: Element) -> Element {
	rsx! {
		div{
			style: "display: flex; flex-direction:column",
			{children}
		}
	}
}

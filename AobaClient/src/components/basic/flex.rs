use dioxus::prelude::*;

#[component]
pub fn FlexRow(children: Element) -> Element
{
	rsx! {
		div{
			style: "display: flex; flex-direction:row",
			{children}
		}
	}
}

#[component]
pub fn FlexColumn(children: Element, gap: Option<String>) -> Element
{
	let gap = gap.map(|g| format!("gap: {};", g)).unwrap_or_default();
	rsx! {
		div{
			style: "display: flex; flex-direction:column; {gap}",
			{children}
		}
	}
}

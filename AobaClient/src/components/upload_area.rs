use dioxus::prelude::*;

pub fn UploadArea(children: Element) -> Element {
	rsx! {
		div{
			id: "",
			{children}
		}
	}
}

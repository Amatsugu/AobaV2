use dioxus::prelude::*;

#[component]
pub fn Info() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "currentColor",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				clip_rule: "evenodd",
				d: "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm8.706-1.442c1.146-.573 2.437.463 2.126 1.706l-.709 2.836.042-.02a.75.75 0 0 1 .67 1.34l-.04.022c-1.147.573-2.438-.463-2.127-1.706l.71-2.836-.042.02a.75.75 0 1 1-.671-1.34l.041-.022ZM12 9a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z",
				fill_rule: "evenodd",
			}
		}
	}
}

#[component]
pub fn Warn() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "currentColor",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				clip_rule: "evenodd",
				d: "M9.401 3.003c1.155-2 4.043-2 5.197 0l7.355 12.748c1.154 2-.29 4.5-2.599 4.5H4.645c-2.309 0-3.752-2.5-2.598-4.5L9.4 3.003ZM12 8.25a.75.75 0 0 1 .75.75v3.75a.75.75 0 0 1-1.5 0V9a.75.75 0 0 1 .75-.75Zm0 8.25a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z",
				fill_rule: "evenodd",
			}
		}
	}
}

#[component]
pub fn Error() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "currentColor",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				clip_rule: "evenodd",
				d: "M9.401 3.003c1.155-2 4.043-2 5.197 0l7.355 12.748c1.154 2-.29 4.5-2.599 4.5H4.645c-2.309 0-3.752-2.5-2.598-4.5L9.4 3.003ZM12 8.25a.75.75 0 0 1 .75.75v3.75a.75.75 0 0 1-1.5 0V9a.75.75 0 0 1 .75-.75Zm0 8.25a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z",
				fill_rule: "evenodd",
			}
		}
	}
}

#[component]
pub fn Image() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "currentColor",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				clip_rule: "evenodd",
				d: "M1.5 6a2.25 2.25 0 0 1 2.25-2.25h16.5A2.25 2.25 0 0 1 22.5 6v12a2.25 2.25 0 0 1-2.25 2.25H3.75A2.25 2.25 0 0 1 1.5 18V6ZM3 16.06V18c0 .414.336.75.75.75h16.5A.75.75 0 0 0 21 18v-1.94l-2.69-2.689a1.5 1.5 0 0 0-2.12 0l-.88.879.97.97a.75.75 0 1 1-1.06 1.06l-5.16-5.159a1.5 1.5 0 0 0-2.12 0L3 16.061Zm10.125-7.81a1.125 1.125 0 1 1 2.25 0 1.125 1.125 0 0 1-2.25 0Z",
				fill_rule: "evenodd",
			}
		}
	}
}

#[component]
pub fn Cog() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "currentColor",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				clip_rule: "evenodd",
				d: "M11.828 2.25c-.916 0-1.699.663-1.85 1.567l-.091.549a.798.798 0 0 1-.517.608 7.45 7.45 0 0 0-.478.198.798.798 0 0 1-.796-.064l-.453-.324a1.875 1.875 0 0 0-2.416.2l-.243.243a1.875 1.875 0 0 0-.2 2.416l.324.453a.798.798 0 0 1 .064.796 7.448 7.448 0 0 0-.198.478.798.798 0 0 1-.608.517l-.55.092a1.875 1.875 0 0 0-1.566 1.849v.344c0 .916.663 1.699 1.567 1.85l.549.091c.281.047.508.25.608.517.06.162.127.321.198.478a.798.798 0 0 1-.064.796l-.324.453a1.875 1.875 0 0 0 .2 2.416l.243.243c.648.648 1.67.733 2.416.2l.453-.324a.798.798 0 0 1 .796-.064c.157.071.316.137.478.198.267.1.47.327.517.608l.092.55c.15.903.932 1.566 1.849 1.566h.344c.916 0 1.699-.663 1.85-1.567l.091-.549a.798.798 0 0 1 .517-.608 7.52 7.52 0 0 0 .478-.198.798.798 0 0 1 .796.064l.453.324a1.875 1.875 0 0 0 2.416-.2l.243-.243c.648-.648.733-1.67.2-2.416l-.324-.453a.798.798 0 0 1-.064-.796c.071-.157.137-.316.198-.478.1-.267.327-.47.608-.517l.55-.091a1.875 1.875 0 0 0 1.566-1.85v-.344c0-.916-.663-1.699-1.567-1.85l-.549-.091a.798.798 0 0 1-.608-.517 7.507 7.507 0 0 0-.198-.478.798.798 0 0 1 .064-.796l.324-.453a1.875 1.875 0 0 0-.2-2.416l-.243-.243a1.875 1.875 0 0 0-2.416-.2l-.453.324a.798.798 0 0 1-.796.064 7.462 7.462 0 0 0-.478-.198.798.798 0 0 1-.517-.608l-.091-.55a1.875 1.875 0 0 0-1.85-1.566h-.344ZM12 15.75a3.75 3.75 0 1 0 0-7.5 3.75 3.75 0 0 0 0 7.5Z",
				fill_rule: "evenodd",
			}
		}
	}
}

#[component]
pub fn Archive() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "1.5",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				d: "m20.25 7.5-.625 10.632a2.25 2.25 0 0 1-2.247 2.118H6.622a2.25 2.25 0 0 1-2.247-2.118L3.75 7.5M10 11.25h4M3.375 7.5h17.25c.621 0 1.125-.504 1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125Z",
				stroke_linecap: "round",
				stroke_linejoin: "round",
			}
		}
	}
}

#[component]
pub fn Squares() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "1.5",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				d: "M3.75 6A2.25 2.25 0 0 1 6 3.75h2.25A2.25 2.25 0 0 1 10.5 6v2.25a2.25 2.25 0 0 1-2.25 2.25H6a2.25 2.25 0 0 1-2.25-2.25V6ZM3.75 15.75A2.25 2.25 0 0 1 6 13.5h2.25a2.25 2.25 0 0 1 2.25 2.25V18a2.25 2.25 0 0 1-2.25 2.25H6A2.25 2.25 0 0 1 3.75 18v-2.25ZM13.5 6a2.25 2.25 0 0 1 2.25-2.25H18A2.25 2.25 0 0 1 20.25 6v2.25A2.25 2.25 0 0 1 18 10.5h-2.25a2.25 2.25 0 0 1-2.25-2.25V6ZM13.5 15.75a2.25 2.25 0 0 1 2.25-2.25H18a2.25 2.25 0 0 1 2.25 2.25V18A2.25 2.25 0 0 1 18 20.25h-2.25A2.25 2.25 0 0 1 13.5 18v-2.25Z",
				stroke_linecap: "round",
				stroke_linejoin: "round",
			}
		}
	}
}

#[component]
pub fn Stack(filled: Option<bool>) -> Element
{
	if filled.unwrap_or_default()
	{
		return rsx! {
			svg {
				class: "size-6",
				fill: "currentColor",
				view_box: "0 0 24 24",
				xmlns: "http://www.w3.org/2000/svg",
				path { d: "M16.5 6a3 3 0 0 0-3-3H6a3 3 0 0 0-3 3v7.5a3 3 0 0 0 3 3v-6A4.5 4.5 0 0 1 10.5 6h6Z" }
				path { d: "M18 7.5a3 3 0 0 1 3 3V18a3 3 0 0 1-3 3h-7.5a3 3 0 0 1-3-3v-7.5a3 3 0 0 1 3-3H18Z" }
			}
		};
	}
	rsx! {
		svg {
			class: "size-6",
			fill: "currentColor",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path { d: "M11.644 1.59a.75.75 0 0 1 .712 0l9.75 5.25a.75.75 0 0 1 0 1.32l-9.75 5.25a.75.75 0 0 1-.712 0l-9.75-5.25a.75.75 0 0 1 0-1.32l9.75-5.25Z" }
			path { d: "m3.265 10.602 7.668 4.129a2.25 2.25 0 0 0 2.134 0l7.668-4.13 1.37.739a.75.75 0 0 1 0 1.32l-9.75 5.25a.75.75 0 0 1-.71 0l-9.75-5.25a.75.75 0 0 1 0-1.32l1.37-.738Z" }
			path { d: "m10.933 19.231-7.668-4.13-1.37.739a.75.75 0 0 0 0 1.32l9.75 5.25c.221.12.489.12.71 0l9.75-5.25a.75.75 0 0 0 0-1.32l-1.37-.738-7.668 4.13a2.25 2.25 0 0 1-2.134-.001Z" }
		}
	}
}

#[component]
pub fn Cross() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "1.5",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				d: "M6 18 18 6M6 6l12 12",
				stroke_linecap: "round",
				stroke_linejoin: "round",
			}
		}
	}
}

#[component]
pub fn UpCloud() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "currentColor",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				clip_rule: "evenodd",
				d: "M10.5 3.75a6 6 0 0 0-5.98 6.496A5.25 5.25 0 0 0 6.75 20.25H18a4.5 4.5 0 0 0 2.206-8.423 3.75 3.75 0 0 0-4.133-4.303A6.001 6.001 0 0 0 10.5 3.75Zm2.03 5.47a.75.75 0 0 0-1.06 0l-3 3a.75.75 0 1 0 1.06 1.06l1.72-1.72v4.94a.75.75 0 0 0 1.5 0v-4.94l1.72 1.72a.75.75 0 1 0 1.06-1.06l-3-3Z",
				fill_rule: "evenodd",
			}
		}
	}
}

#[component]
pub fn LeftChevron() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "1.5",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				d: "M15.75 19.5 8.25 12l7.5-7.5",
				stroke_linecap: "round",
				stroke_linejoin: "round",
			}
		}
	}
}

#[component]
pub fn RightChevron() -> Element
{
	rsx! {
	  svg {
			class: "size-6",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "1.5",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				d: "m8.25 4.5 7.5 7.5-7.5 7.5",
				stroke_linecap: "round",
				stroke_linejoin: "round",
			}
		}
	}
}

#[component]
pub fn NewTabArrowIcon() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "1.5",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				d: "M13.5 6H5.25A2.25 2.25 0 0 0 3 8.25v10.5A2.25 2.25 0 0 0 5.25 21h10.5A2.25 2.25 0 0 0 18 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25",
				stroke_linecap: "round",
				stroke_linejoin: "round",
			}
		}
	}
}

#[component]
pub fn DownloadFilled() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "currentColor",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				clip_rule: "evenodd",
				d: "M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25Zm-.53 14.03a.75.75 0 0 0 1.06 0l3-3a.75.75 0 1 0-1.06-1.06l-1.72 1.72V8.25a.75.75 0 0 0-1.5 0v5.69l-1.72-1.72a.75.75 0 0 0-1.06 1.06l3 3Z",
				fill_rule: "evenodd",
			}
		}
	}
}

#[component]
pub fn TrashFilled() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "currentColor",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				clip_rule: "evenodd",
				d: "M16.5 4.478v.227a48.816 48.816 0 0 1 3.878.512.75.75 0 1 1-.256 1.478l-.209-.035-1.005 13.07a3 3 0 0 1-2.991 2.77H8.084a3 3 0 0 1-2.991-2.77L4.087 6.66l-.209.035a.75.75 0 0 1-.256-1.478A48.567 48.567 0 0 1 7.5 4.705v-.227c0-1.564 1.213-2.9 2.816-2.951a52.662 52.662 0 0 1 3.369 0c1.603.051 2.815 1.387 2.815 2.951Zm-6.136-1.452a51.196 51.196 0 0 1 3.273 0C14.39 3.05 15 3.684 15 4.478v.113a49.488 49.488 0 0 0-6 0v-.113c0-.794.609-1.428 1.364-1.452Zm-.355 5.945a.75.75 0 1 0-1.5.058l.347 9a.75.75 0 1 0 1.499-.058l-.346-9Zm5.48.058a.75.75 0 1 0-1.498-.058l-.347 9a.75.75 0 0 0 1.5.058l.345-9Z",
				fill_rule: "evenodd",
			}
		}
	}
}

#[component]
pub fn FireFilled() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "currentColor",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path {
				clip_rule: "evenodd",
				d: "M12.963 2.286a.75.75 0 0 0-1.071-.136 9.742 9.742 0 0 0-3.539 6.176 7.547 7.547 0 0 1-1.705-1.715.75.75 0 0 0-1.152-.082A9 9 0 1 0 15.68 4.534a7.46 7.46 0 0 1-2.717-2.248ZM15.75 14.25a3.75 3.75 0 1 1-7.313-1.172c.628.465 1.35.81 2.133 1a5.99 5.99 0 0 1 1.925-3.546 3.75 3.75 0 0 1 3.255 3.718Z",
				fill_rule: "evenodd",
			}
		}
	}
}

#[component]
pub fn HiddenEyeFilled() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "currentColor",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path { d: "M3.53 2.47a.75.75 0 0 0-1.06 1.06l18 18a.75.75 0 1 0 1.06-1.06l-18-18ZM22.676 12.553a11.249 11.249 0 0 1-2.631 4.31l-3.099-3.099a5.25 5.25 0 0 0-6.71-6.71L7.759 4.577a11.217 11.217 0 0 1 4.242-.827c4.97 0 9.185 3.223 10.675 7.69.12.362.12.752 0 1.113Z" }
			path { d: "M15.75 12c0 .18-.013.357-.037.53l-4.244-4.243A3.75 3.75 0 0 1 15.75 12ZM12.53 15.713l-4.243-4.244a3.75 3.75 0 0 0 4.244 4.243Z" }
			path { d: "M6.75 12c0-.619.107-1.213.304-1.764l-3.1-3.1a11.25 11.25 0 0 0-2.63 4.31c-.12.362-.12.752 0 1.114 1.489 4.467 5.704 7.69 10.675 7.69 1.5 0 2.933-.294 4.242-.827l-2.477-2.477A5.25 5.25 0 0 1 6.75 12Z" }
		}
	}
}

#[component]
pub fn EyeFilled() -> Element
{
	rsx! {
		svg {
			class: "size-6",
			fill: "currentColor",
			view_box: "0 0 24 24",
			xmlns: "http://www.w3.org/2000/svg",
			path { d: "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z" }
			path {
				clip_rule: "evenodd",
				d: "M1.323 11.447C2.811 6.976 7.028 3.75 12.001 3.75c4.97 0 9.185 3.223 10.675 7.69.12.362.12.752 0 1.113-1.487 4.471-5.705 7.697-10.677 7.697-4.97 0-9.186-3.223-10.675-7.69a1.762 1.762 0 0 1 0-1.113ZM17.25 12a5.25 5.25 0 1 1-10.5 0 5.25 5.25 0 0 1 10.5 0Z",
				fill_rule: "evenodd",
			}
		}
	}
}

#[component]
pub fn Check(filled: Option<bool>) -> Element
{
	if filled.unwrap_or_default()
	{
		rsx! {
			svg {
				class: "size-6",
				fill: "currentColor",
				view_box: "0 0 24 24",
				xmlns: "http://www.w3.org/2000/svg",
				path {
					clip_rule: "evenodd",
					d: "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm13.36-1.814a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z",
					fill_rule: "evenodd",
				}
			}
		}
	}
	else
	{
		rsx! {
			svg {
				class: "size-6",
				fill: "none",
				stroke: "currentColor",
				stroke_width: "1.5",
				view_box: "0 0 24 24",
				xmlns: "http://www.w3.org/2000/svg",
				path {
					d: "M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
					stroke_linecap: "round",
					stroke_linejoin: "round",
				}
			}
		}
	}
}

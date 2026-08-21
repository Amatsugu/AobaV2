use dioxus::prelude::*;

use crate::{
	Route,
	components::{
		Navbar,
		toasts::{ToastsDisplay, init_toasts},
	},
	contexts::{AuthContext, DragContext},
	views::Login,
};

#[component]
pub fn MainLayout() -> Element
{
	let auth_context = use_context::<AuthContext>();
	let mut drag_context = use_context::<DragContext>();

	let ctx = init_toasts();
	use_context_provider(|| ctx);

	if auth_context.jwt.cloned().is_none()
	{
		return rsx! {
			Login { }
		};
	}

	let on_drag_enter = move |_| {
		drag_context.is_dragging.set(true);
	};
	return rsx! {
		// ContextMenuRoot {  }
		ToastsDisplay {}
		Navbar { }
		div {
			id: "content",
			ondragenter: on_drag_enter,
			ondragover: on_drag_enter,
			ondragstart: on_drag_enter,
			// ondragexit: on_drag_exit,
			// ondragend: on_drag_exit,
			// ondragleave: on_drag_exit,
			Outlet::<Route> { }
		}
	};
}

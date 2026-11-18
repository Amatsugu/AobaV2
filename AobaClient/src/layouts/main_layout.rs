use dioxus::prelude::*;

use crate::{
	Route,
	components::{Navbar, toasts::ToastsDisplay},
	contexts::{AuthContext, DragContext},
	rpc::use_unauth_hook,
	views::Login,
};

#[component]
pub fn MainLayout() -> Element
{
	let auth_context = use_context::<AuthContext>();
	let mut drag_context = use_context::<DragContext>();

	let on_drag_enter = move |_| {
		drag_context.is_dragging.set(true);
	};

	use_unauth_hook(auth_context);

	if !auth_context.logged_in.cloned()
	{
		return rsx! {
			Login { }
		};
	}
	return rsx! {
		ToastsDisplay {}
		Navbar { }
		div {
			id: "content",
			ondragenter: on_drag_enter,
			ondragover: on_drag_enter,
			ondragstart: on_drag_enter,

			Outlet::<Route> { }
		}
	};
}

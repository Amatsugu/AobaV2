use dioxus::prelude::*;
use tokio::sync::mpsc::unbounded_channel;

use crate::{
	Route,
	components::{Navbar, toasts::ToastsDisplay},
	contexts::{AuthContext, DragContext},
	rpc::UNAUTH_TX,
	views::Login,
};

#[component]
pub fn MainLayout() -> Element {
	let mut auth_context = use_context::<AuthContext>();
	let mut drag_context = use_context::<DragContext>();
	use_hook(|| {
		let (tx, mut rx) = unbounded_channel::<()>();
		let _ = UNAUTH_TX.set(tx);
		spawn(async move {
			while rx.recv().await.is_some() {
				auth_context.logout();
			}
		});
	});

	if auth_context.jwt.cloned().is_none() {
		return rsx! {
			Login { }
		};
	}
	let on_drag_enter = move |_| {
		drag_context.is_dragging.set(true);
	};
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

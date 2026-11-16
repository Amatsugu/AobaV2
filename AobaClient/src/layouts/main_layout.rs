use std::{
	sync::atomic::{AtomicU64, Ordering},
	time::Duration,
};

use dioxus::prelude::*;

use crate::{
	Route,
	components::{Navbar, toasts::ToastsDisplay},
	contexts::AuthContext,
	rpc::{get_account_rpc_client, use_unauth_hook},
	views::Login,
};

static LAST_ACTIVITY: AtomicU64 = AtomicU64::new(0);

pub fn mark_activity() {
	LAST_ACTIVITY.store(js_sys::Date::now() as u64, Ordering::Relaxed);
}

#[component]
pub fn MainLayout() -> Element {
	let auth_context = use_context::<AuthContext>();

	use_unauth_hook(auth_context);
	init_session_refresh(auth_context);

	if auth_context.jwt.cloned().is_none() {
		return rsx! {
			Login { }
		};
	}
	return rsx! {
		ToastsDisplay {}
		Navbar { }
		div {
			id: "content",
			//Activity
			onmousemove: |_| mark_activity(),
			onpointerdown: |_| mark_activity(),
			onscroll: |_| mark_activity(),
			onkeydown: |_| mark_activity(),
			Outlet::<Route> { }
		}
	};
}

fn init_session_refresh(mut auth_context: AuthContext) {
	use_resource(move || {
		let token = auth_context.jwt.cloned();
		async move {
			let Some(token) = token else {
				auth_context.logout();
				return;
			};

			let mut rpc = get_account_rpc_client();
			let refresh = rpc.refresh_session(()).await;
			let refresh = refresh
				.ok()
				.map(|e| {
					e.into_inner().result.and_then(|r| {
						use crate::rpc::aoba::login_response::Result;
						match r {
							Result::Jwt(jwt) => Some(jwt),
							Result::Error(_) => None,
						}
					})
				})
				.flatten();
			match refresh {
				Some(token) => auth_context.jwt.set(Some(token)),
				None => auth_context.logout(),
			}
		}
	});
}

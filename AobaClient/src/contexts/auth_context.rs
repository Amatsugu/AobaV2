use dioxus::signals::{Signal, WritableExt};
use web_sys::window;

use crate::rpc::{aoba::Jwt, login, logout};

#[derive(Clone, Copy, Default)]
pub struct AuthContext {
	pub jwt: Signal<Option<Jwt>>,
}

impl AuthContext {
	pub fn login(&mut self, jwt: Jwt) {
		self.jwt.set(Some(jwt.clone()));
		if window()
			.and_then(|w| w.local_storage().ok())
			.flatten()
			.and_then(|l| l.set_item("token", &jwt.token.as_str()).ok())
			.is_some()
		{
			login(jwt.token.clone());
		}
	}

	pub fn logout(&mut self) {
		self.jwt.set(None);
		_ = window()
			.and_then(|w| w.local_storage().ok())
			.flatten()
			.and_then(|l| l.remove_item("token").ok());
		logout();
	}

	pub fn new_from_session() -> Self {
		match window()
			.and_then(|w| w.local_storage().ok())
			.flatten()
			.and_then(|l| l.get_item("token").ok())
			.flatten()
		{
			Some(jwt) => {
				login(jwt.clone());
				AuthContext {
					jwt: Signal::new(Some(Jwt { token: jwt, exp: 0 })),
					//todo: store + retreive exp
				}
			}
			_ => AuthContext::default(),
		}
	}
}

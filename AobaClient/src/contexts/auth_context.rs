use dioxus::signals::{Signal, WritableExt};
use web_sys::window;

use crate::rpc::{login, logout};

#[derive(Clone, Copy, Default)]
pub struct AuthContext {
	pub jwt: Signal<Option<String>>,
}

impl AuthContext {
	pub fn login(&mut self, token: String) {
		self.jwt.set(Some(token.clone()));
		let local_storage = window().unwrap().local_storage().unwrap().unwrap();
		_ = local_storage.set_item("token", token.as_str());
		login(token.clone());
	}

	pub fn logout(&mut self) {
		self.jwt.set(None);
		let local_storage = window().unwrap().local_storage().unwrap().unwrap();
		_ = local_storage.remove_item("token");
		logout();
	}

	pub fn new() -> Self {
		println!("new");
		let local_storage = window().unwrap().local_storage().unwrap().unwrap();
		match local_storage.get_item("token") {
			Ok(value) => {
				if let Some(jwt) = value {
					login(jwt.clone());
					return AuthContext {
						jwt: Signal::new(Some(jwt)),
					};
				}
				return AuthContext::default();
			}
			Err(_) => AuthContext::default(),
		}
	}
}

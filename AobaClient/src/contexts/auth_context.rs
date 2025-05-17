use dioxus::signals::{Signal, Writable};
use web_sys::window;

#[derive(Clone, Copy, Default)]
pub struct AuthContext {
	pub jwt: Signal<Option<String>>,
}

impl AuthContext {
	pub fn set_token(&mut self, token: String) {
		self.jwt.set(Some(token.clone()));
		let local_storage = window().unwrap().local_storage().unwrap().unwrap();
		_ = local_storage.set_item("token", token.as_str());
	}

	pub fn new() -> Self {
		println!("new");
		let local_storage = window().unwrap().local_storage().unwrap().unwrap();
		match local_storage.get_item("token") {
			Ok(value) => {
				if let Some(jwt) = value {
					println!("jwt");
					return AuthContext {
						jwt: Signal::new(Some(jwt)),
					};
				}
				return AuthContext::default();
			}
			Err(_) => {
				println!("err");
				AuthContext::default()
			}
		}
	}
}

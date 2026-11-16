use dioxus::signals::{Signal, WritableExt};
use serde::{Deserialize, Serialize};
use web_sys::window;

use crate::rpc::{aoba::Jwt, login, logout};

#[derive(Clone, Copy, Default)]
pub struct AuthContext {
	pub jwt: Signal<Option<Jwt>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtJson {
	token: String,
	exp: u64,
}

impl From<&Jwt> for JwtJson {
	fn from(value: &Jwt) -> Self {
		JwtJson {
			token: value.token.clone(),
			exp: value.exp,
		}
	}
}

impl From<JwtJson> for Jwt {
	fn from(value: JwtJson) -> Self {
		Jwt {
			token: value.token,
			exp: value.exp,
		}
	}
}

impl AuthContext {
	pub fn login(&mut self, jwt: Jwt) {
		self.jwt.set(Some(jwt.clone()));
		if window()
			.and_then(|w| w.local_storage().ok())
			.flatten()
			.and_then(|l| l.set_item("token", &Self::jwt_to_json(&jwt)).ok())
			.is_some()
		{
			login(jwt.token.clone());
		}
	}

	fn jwt_to_json(jwt: &Jwt) -> String {
		let jwt: JwtJson = jwt.into();
		serde_json::to_string(&jwt).unwrap_or_default()
	}

	fn json_to_jwt(json: String) -> Option<Jwt> {
		serde_json::from_str(&json).map(|t: JwtJson| t.into()).ok()
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
			.and_then(Self::json_to_jwt)
		{
			Some(jwt) => {
				login(jwt.token.clone());
				AuthContext {
					jwt: Signal::new(Some(jwt)),
				}
			}
			_ => AuthContext::default(),
		}
	}
}

use std::time::{SystemTime, UNIX_EPOCH};

use dioxus::signals::{Signal, WritableExt};
use serde::{Deserialize, Serialize};
use web_sys::window;

use crate::rpc::{aoba::Jwt, login, logout};

#[derive(Clone, Copy, Default)]
pub struct AuthContext
{
	pub logged_in: Signal<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtJson
{
	token: String,
	exp: u64,
}

impl From<&Jwt> for JwtJson
{
	fn from(value: &Jwt) -> Self
	{
		JwtJson {
			token: value.token.clone(),
			exp: value.exp,
		}
	}
}

impl From<JwtJson> for Jwt
{
	fn from(value: JwtJson) -> Self
	{
		Jwt {
			token: value.token,
			exp: value.exp,
		}
	}
}

impl AuthContext
{
	pub fn login(&mut self, jwt: Jwt)
	{
		self.logged_in.set(true);
		if window()
			.and_then(|w| w.local_storage().ok())
			.flatten()
			.and_then(|l| l.set_item("token", &Self::jwt_to_json(&jwt)).ok())
			.is_some()
		{
			login(jwt.token.clone());
		}
	}

	pub fn refresh(&mut self, jwt: Jwt)
	{
		if window()
			.and_then(|w| w.local_storage().ok())
			.flatten()
			.and_then(|l| l.set_item("token", &Self::jwt_to_json(&jwt)).ok())
			.is_some()
		{
			login(jwt.token.clone());
		}
	}

	fn jwt_to_json(jwt: &Jwt) -> String
	{
		let jwt: JwtJson = jwt.into();
		serde_json::to_string(&jwt).unwrap_or_default()
	}

	fn json_to_jwt(json: String) -> Option<Jwt>
	{
		serde_json::from_str(&json).map(|t: JwtJson| t.into()).ok()
	}

	pub fn logout(&mut self)
	{
		self.logged_in.set(false);
		_ = window()
			.and_then(|w| w.local_storage().ok())
			.flatten()
			.and_then(|l| l.remove_item("token").ok());
		logout();
	}

	pub fn new_from_session() -> Self
	{
		match window()
			.and_then(|w| w.local_storage().ok())
			.flatten()
			.and_then(|l| l.get_item("token").ok())
			.flatten()
			.and_then(Self::json_to_jwt)
		{
			Some(jwt) if !is_expired(jwt.exp) =>
			{
				login(jwt.token.clone());
				AuthContext {
					logged_in: Signal::new(true),
				}
			}
			_ => AuthContext::default(),
		}
	}
}

fn is_expired(exp: u64) -> bool
{
	if let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs())
	{
		now >= exp
	}
	else
	{
		true
	}
}

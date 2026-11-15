use dioxus::prelude::*;
use std::sync::RwLock;
use tokio::sync::mpsc::unbounded_channel;
use tonic::service::Interceptor;

use crate::contexts::AuthContext;

static JWT: RwLock<Option<String>> = RwLock::new(None);

#[derive(Clone)]
pub struct AuthInterceptor;
impl Interceptor for AuthInterceptor {
	fn call(
		&mut self,
		mut request: tonic::Request<()>,
	) -> Result<tonic::Request<()>, tonic::Status> {
		match JWT.read().map(|t| t.clone()).ok().flatten() {
			Some(jwt) => {
				if let Ok(bearer) = format!("Bearer {jwt}").parse() {
					request.metadata_mut().insert("authorization", bearer);
				}
				Ok(request)
			}
			None => Err(tonic::Status::new(
				tonic::Code::Unauthenticated,
				"Not logged in",
			)),
		}
	}
}

pub fn login(token: String) {
	if let Ok(mut jwt) = JWT.write() {
		*jwt = Some(token);
	}
}

pub fn logout() {
	if let Ok(mut jwt) = JWT.write() {
		*jwt = None;
	}
}

pub fn use_unauth_hook(mut auth_context: AuthContext) {
	use_hook(|| {
		let (tx, mut rx) = unbounded_channel::<()>();
		let _ = super::UNAUTH_TX.set(tx);
		spawn(async move {
			while rx.recv().await.is_some() {
				auth_context.logout();
			}
		});
	});
}

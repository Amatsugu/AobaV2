use std::sync::RwLock;

use aoba::aoba_rpc_client::AobaRpcClient;
use tonic::service::{Interceptor, interceptor::InterceptedService};
use tonic_web_wasm_client::Client;

use crate::{
	RPC_HOST,
	rpc::aoba::{auth_rpc_client::AuthRpcClient, metrics_rpc_client::MetricsRpcClient},
};

pub mod aoba {
	tonic::include_proto!("aoba");
}

static RPC_CLIENT: RpcConnection = RpcConnection {
	aoba: RwLock::new(None),
	auth: RwLock::new(None),
	metrics: RwLock::new(None),
	jwt: RwLock::new(None),
};

#[derive(Default)]
pub struct RpcConnection {
	aoba: RwLock<Option<AobaRpcClient<InterceptedService<Client, AuthInterceptor>>>>,
	auth: RwLock<Option<AuthRpcClient<Client>>>,
	metrics: RwLock<Option<MetricsRpcClient<InterceptedService<Client, AuthInterceptor>>>>,
	jwt: RwLock<Option<String>>,
}

impl RpcConnection {
	pub fn get_client(&self) -> AobaRpcClient<InterceptedService<Client, AuthInterceptor>> {
		self.ensure_client();
		return self.aoba.read().unwrap().clone().unwrap();
	}

	pub fn get_auth_client(&self) -> AuthRpcClient<Client> {
		self.ensure_client();
		return self.auth.read().unwrap().clone().unwrap();
	}

	pub fn get_metrics_client(&self) -> MetricsRpcClient<InterceptedService<Client, AuthInterceptor>> {
		self.ensure_client();
		return self.metrics.read().unwrap().clone().unwrap();
	}

	fn ensure_client(&self) {
		if self.aoba.read().unwrap().is_none() {
			let wasm_client = Client::new(RPC_HOST.into());
			let aoba_client = AobaRpcClient::with_interceptor(wasm_client.clone(), AuthInterceptor);
			*self.aoba.write().unwrap() = Some(aoba_client);
			*self.auth.write().unwrap() = Some(AuthRpcClient::new(wasm_client.clone()));
			*self.metrics.write().unwrap() =
				Some(MetricsRpcClient::with_interceptor(wasm_client.clone(), AuthInterceptor));
		}
	}
}

#[derive(Clone)]
pub struct AuthInterceptor;
impl Interceptor for AuthInterceptor {
	fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
		if let Some(jwt) = RPC_CLIENT.jwt.read().unwrap().clone() {
			request
				.metadata_mut()
				.insert("authorization", format!("Bearer {jwt}").parse().unwrap());
		}
		return Ok(request);
	}
}

pub fn get_rpc_client() -> AobaRpcClient<InterceptedService<Client, AuthInterceptor>> {
	return RPC_CLIENT.get_client();
}

pub fn get_auth_rpc_client() -> AuthRpcClient<Client> {
	return RPC_CLIENT.get_auth_client();
}

pub fn get_metrics_rpc_client() -> MetricsRpcClient<InterceptedService<Client, AuthInterceptor>> {
	return RPC_CLIENT.get_metrics_client();
}
pub fn login(jwt: String) {
	*RPC_CLIENT.jwt.write().unwrap() = Some(jwt);
}

pub fn logout() {
	*RPC_CLIENT.jwt.write().unwrap() = None;
}

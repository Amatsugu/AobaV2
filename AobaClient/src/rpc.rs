use std::sync::{LazyLock, RwLock};

use aoba::aoba_rpc_client::AobaRpcClient;
use tonic::service::{Interceptor, interceptor::InterceptedService};
use tonic_web_wasm_client::Client;

use crate::{
	RPC_HOST,
	rpc::aoba::{
		account_rpc_client::AccountRpcClient, auth_rpc_client::AuthRpcClient, metrics_rpc_client::MetricsRpcClient,
	},
};

pub mod aoba
{
	tonic::include_proto!("aoba");
}

static JWT: RwLock<Option<String>> = RwLock::new(None);

static RPC_CLIENTS: LazyLock<RpcConnection> = LazyLock::new(|| {
	let wasm_client = Client::new(RPC_HOST.into());
	RpcConnection {
		aoba: AobaRpcClient::with_interceptor(wasm_client.clone(), AuthInterceptor),
		auth: AuthRpcClient::new(wasm_client.clone()),
		account: AccountRpcClient::with_interceptor(wasm_client.clone(), AuthInterceptor),
		metrics: MetricsRpcClient::with_interceptor(wasm_client.clone(), AuthInterceptor),
	}
});

pub struct RpcConnection
{
	aoba: AobaRpcClient<InterceptedService<Client, AuthInterceptor>>,
	auth: AuthRpcClient<Client>,
	account: AccountRpcClient<InterceptedService<Client, AuthInterceptor>>,
	metrics: MetricsRpcClient<InterceptedService<Client, AuthInterceptor>>,
}

impl RpcConnection
{
	pub fn get_client(&self) -> AobaRpcClient<InterceptedService<Client, AuthInterceptor>>
	{
		self.aoba.clone()
	}

	pub fn get_account_client(&self) -> AccountRpcClient<InterceptedService<Client, AuthInterceptor>>
	{
		self.account.clone()
	}

	pub fn get_auth_client(&self) -> AuthRpcClient<Client>
	{
		self.auth.clone()
	}

	pub fn get_metrics_client(&self) -> MetricsRpcClient<InterceptedService<Client, AuthInterceptor>>
	{
		self.metrics.clone()
	}
}

#[derive(Clone)]
pub struct AuthInterceptor;
impl Interceptor for AuthInterceptor
{
	fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>
	{
		if let Some(jwt) = JWT.read().unwrap().clone()
		{
			request
				.metadata_mut()
				.insert("authorization", format!("Bearer {jwt}").parse().unwrap());
		}
		return Ok(request);
	}
}

pub fn get_rpc_client() -> AobaRpcClient<InterceptedService<Client, AuthInterceptor>>
{
	return RPC_CLIENTS.get_client();
}

pub fn get_auth_rpc_client() -> AuthRpcClient<Client>
{
	return RPC_CLIENTS.get_auth_client();
}

pub fn get_account_rpc_client() -> AccountRpcClient<InterceptedService<Client, AuthInterceptor>>
{
	return RPC_CLIENTS.get_account_client();
}

pub fn get_metrics_rpc_client() -> MetricsRpcClient<InterceptedService<Client, AuthInterceptor>>
{
	return RPC_CLIENTS.get_metrics_client();
}
pub fn login(jwt: String)
{
	*JWT.write().unwrap() = Some(jwt);
}

pub fn logout()
{
	*JWT.write().unwrap() = None;
}

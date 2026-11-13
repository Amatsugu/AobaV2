use std::sync::{LazyLock, OnceLock};

use aoba::aoba_rpc_client::AobaRpcClient;
use tokio::sync::mpsc::UnboundedSender;
use tonic::service::interceptor::InterceptedService;
use tonic_web_wasm_client::Client;
use tower_layer::Layer;

use crate::{
	RPC_HOST,
	rpc::{
		aoba::{
			account_rpc_client::AccountRpcClient, auth_rpc_client::AuthRpcClient,
			metrics_rpc_client::MetricsRpcClient,
		},
		auth::AuthInterceptor,
		unauth::{UnAuthLayer, UnauthService},
	},
};

pub mod aoba {
	tonic::include_proto!("aoba");
}

type AuthorizedClient = InterceptedService<UnauthService<Client>, AuthInterceptor>;
pub static UNAUTH_TX: OnceLock<UnboundedSender<()>> = OnceLock::new();
static RPC_CLIENTS: LazyLock<RpcConnection> = LazyLock::new(|| {
	let wasm_client = Client::new(RPC_HOST.into());
	let unauth = UnAuthLayer(on_unauth).layer(wasm_client.clone());
	RpcConnection {
		aoba: AobaRpcClient::with_interceptor(unauth.clone(), AuthInterceptor),
		auth: AuthRpcClient::new(wasm_client.clone()),
		account: AccountRpcClient::with_interceptor(unauth.clone(), AuthInterceptor),
		metrics: MetricsRpcClient::with_interceptor(unauth.clone(), AuthInterceptor),
	}
});

pub struct RpcConnection {
	aoba: AobaRpcClient<AuthorizedClient>,
	auth: AuthRpcClient<Client>,
	account: AccountRpcClient<AuthorizedClient>,
	metrics: MetricsRpcClient<AuthorizedClient>,
}

impl RpcConnection {
	pub fn get_client(&self) -> AobaRpcClient<AuthorizedClient> {
		self.aoba.clone()
	}

	pub fn get_account_client(&self) -> AccountRpcClient<AuthorizedClient> {
		self.account.clone()
	}

	pub fn get_auth_client(&self) -> AuthRpcClient<Client> {
		self.auth.clone()
	}

	pub fn get_metrics_client(&self) -> MetricsRpcClient<AuthorizedClient> {
		self.metrics.clone()
	}
}

pub fn get_rpc_client() -> AobaRpcClient<AuthorizedClient> {
	RPC_CLIENTS.get_client()
}

pub fn get_auth_rpc_client() -> AuthRpcClient<Client> {
	RPC_CLIENTS.get_auth_client()
}

pub fn get_account_rpc_client() -> AccountRpcClient<AuthorizedClient> {
	RPC_CLIENTS.get_account_client()
}

pub fn get_metrics_rpc_client() -> MetricsRpcClient<AuthorizedClient> {
	RPC_CLIENTS.get_metrics_client()
}

fn on_unauth() {
	if let Some(tx) = UNAUTH_TX.get() {
		let _ = tx.send(());
	}
}

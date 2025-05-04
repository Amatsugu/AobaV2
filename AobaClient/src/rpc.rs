use std::sync::RwLock;

use aoba::{aoba_rpc_client::AobaRpcClient, auth_rpc_client::AuthRpcClient};
use tonic_web_wasm_client::Client;

use crate::HOST;

pub mod aoba {
	tonic::include_proto!("aoba");
	tonic::include_proto!("aoba.auth");
}

static RPC_CLIENT: RpcConnection = RpcConnection {
	aoba: RwLock::new(None),
	auth: RwLock::new(None),
};

#[derive(Default)]
pub struct RpcConnection {
	aoba: RwLock<Option<AobaRpcClient<Client>>>,
	auth: RwLock<Option<AuthRpcClient<Client>>>,
}

impl RpcConnection {
	pub fn get_client(&self) -> AobaRpcClient<Client> {
		self.ensure_client();
		return self.aoba.read().unwrap().clone().unwrap();
	}

	pub fn get_auth_client(&self) -> AuthRpcClient<Client> {
		self.ensure_client();
		return self.auth.read().unwrap().clone().unwrap();
	}

	fn ensure_client(&self) {
		if self.aoba.read().unwrap().is_none() {
			let wasm_client = Client::new(HOST.into());
			*self.aoba.write().unwrap() = Some(AobaRpcClient::new(wasm_client.clone()));
			*self.auth.write().unwrap() = Some(AuthRpcClient::new(wasm_client.clone()));
		}
	}
}

pub fn get_rpc_client() -> AobaRpcClient<Client> {
	return RPC_CLIENT.get_client();
}

pub fn get_auth_rpc_client() -> AuthRpcClient<Client> {
	return RPC_CLIENT.get_auth_client();
}

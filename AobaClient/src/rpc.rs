use std::sync::RwLock;

use aoba::aoba_rpc_client::AobaRpcClient;
use tonic_web_wasm_client::Client;

pub mod aoba {
	tonic::include_proto!("aoba");
}

static RPC_CLIENT: RpcConnection = RpcConnection {
	client: RwLock::new(None),
};

#[derive(Default)]
pub struct RpcConnection {
	client: RwLock<Option<AobaRpcClient<Client>>>,
}

impl RpcConnection {
	pub fn get_client(&self) -> AobaRpcClient<Client> {
		self.ensure_client();
		return self.client.read().unwrap().clone().unwrap();
	}

	fn ensure_client(&self) {
		if self.client.read().unwrap().is_none() {
			let wasm_client = Client::new("http://localhost:5164".into());
			let c = AobaRpcClient::new(wasm_client);
			*self.client.write().unwrap() = Some(c);
		}
	}
}

pub fn get_rpc_client() -> AobaRpcClient<Client> {
	return RPC_CLIENT.get_client();
}

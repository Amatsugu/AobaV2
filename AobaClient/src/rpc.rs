use std::sync::RwLock;

use aoba::aoba_rpc_client::AobaRpcClient;
use tonic::transport::Channel;

pub mod aoba {
	tonic::include_proto!("aoba");
}

static RPC_CLIENT: RpcConnection = RpcConnection {
	client: RwLock::new(None),
};

#[derive(Default)]
pub struct RpcConnection {
	client: RwLock<Option<AobaRpcClient<Channel>>>,
}

impl RpcConnection {
	pub async fn get_client(&self) -> AobaRpcClient<Channel> {
		self.ensure_client().await;
		return self.client.read().unwrap().clone().unwrap();
	}

	async fn ensure_client(&self) {
		if self.client.read().unwrap().is_none() {
			let c = AobaRpcClient::connect("http://localhost:5000")
				.await
				.expect("Failed to connect RPC");
			*self.client.write().unwrap() = Some(c);
		}
	}
}

pub async fn get_rpc_client() -> AobaRpcClient<Channel> {
	return RPC_CLIENT.get_client().await;
}

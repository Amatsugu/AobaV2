use dioxus::{html::FileData, prelude::*};
use tokio::sync::RwLock;

use crate::file_uploader::UploadState;

pub struct UploaderContext {
	pub upload_state: Signal<UploadState>,
	pub upload_queue: RwLock<Vec<FileData>>,
}

impl UploaderContext {
	pub async fn start_upload(&mut self, files: Vec<FileData>) {
		{
			let mut queue = self.upload_queue.write().await;
			for file in files {
				queue.push(file);
			}
		}

		todo!()
	}
}

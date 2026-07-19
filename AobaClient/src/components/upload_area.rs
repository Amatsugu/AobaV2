use dioxus::{html::HasFileData, prelude::*};

use crate::rpc::{
	aoba::{FileInfo, UploadRequest, upload_target_response::Result},
	get_rpc_client,
};

#[derive(PartialEq, Clone, Props)]
pub struct UploadAreaProps
{
	children: Element,
}

enum UploadState
{
	Idle,
	PreparingUpload,
	Uploading(usize),
	Done,
}

#[component]
pub fn UploadArea(props: UploadAreaProps) -> Element
{
	let mut is_dragging = use_signal(|| false);
	let mut upload_state = use_signal(|| UploadState::Idle);
	let mut file_count = use_signal(|| None::<usize>);
	let on_drag_enter = move |_e: Event<DragData>| {
		is_dragging.set(true);
		println!("Hover");
	};
	let on_drag_exit = move |_e: Event<DragData>| {
		is_dragging.set(false);
		println!("Hover Exit");
	};
	let on_files_dropped = move |e: Event<DragData>| {
		file_count.set(Some(e.files().len()));
		let upload_request = UploadRequest {
			files: e
				.files()
				.iter()
				.map(|f| FileInfo {
					filename: f.name(),
					size: f.size(),
				})
				.collect(),
		};
		spawn(async move {
			let mut client = get_rpc_client();
			let result = client.start_upload(upload_request).await;
			if let Ok(response) = result
				&& let Some(targets) = response.into_inner().result
			{
				match targets
				{
					Result::Targets(upload_targets) =>
					{
						let files = e.files();
						let client = reqwest::Client::new();
						for file_tgt in upload_targets.targets
						{
							let file = files.iter().find(|f| f.name() == file_tgt.filename);
							if let Some(file) = file
								&& let Ok(bytes) = file.read_bytes().await
							{
								let upload_result = client
									.post(file_tgt.signed_url)
									.header("Content-Length", bytes.len())
									.body(bytes)
									.send()
									.await;
								if let Ok(upload) = upload_result
								{}
							}
						}
					}
					Result::Error(_) => todo!("Handle target creation error"),
				}
			}
		});
	};
	rsx! {
		div{
			id: "",
			ondragenter: on_drag_enter,
			ondragexit: on_drag_exit,
			ondragend: on_drag_exit,
			ondragleave: on_drag_exit,
			ondrop: on_files_dropped,
			{props.children}
			if is_dragging.cloned() {
				DragOverlay {}
			}
		}
	}
}

#[component]
pub fn DragOverlay() -> Element
{
	rsx! {
		"Hover"
	}
}

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
	let mut upload_progress = use_signal(|| 0.0_f32);
	let on_drag_enter = move |_e: Event<DragData>| {
		if !is_dragging.cloned()
		{
			is_dragging.set(true);
		}
		info!("Hover");
	};
	let on_drag_exit = move |_e: Event<DragData>| {
		if is_dragging.cloned()
		{
			is_dragging.set(false);
		}
		info!("Hover Exit");
	};
	let on_files_dropped = move |e: Event<DragData>| {
		is_dragging.set(false);
		e.prevent_default();
		file_count.set(Some(e.files().len()));
		let total_file_size: u64 = e.files().iter().map(|f| f.size()).sum();
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

	let visibility = use_memo(move || match is_dragging()
	{
		true => "opacity: 1;",
		false => "opacity:0;",
	});
	rsx! {
		div{
			id: "uploadArea",
			ondragenter: on_drag_enter,
			ondragover: on_drag_enter,
			ondragstart: on_drag_enter,
			ondragexit: on_drag_exit,
			ondragend: on_drag_exit,
			ondragleave: on_drag_exit,
			ondrop: on_files_dropped,
			div{
				{props.children}
			}
			div{
				class: "dragOverlay",
				style: visibility(),
				DragOverlay {
				}
			}
		}
	}
}

#[component]
pub fn DragOverlay() -> Element
{
	rsx! {
		div{
			"Hover"
		}
	}
}

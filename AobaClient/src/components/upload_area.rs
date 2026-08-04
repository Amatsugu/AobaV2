use dioxus::{
	html::{FileData, HasFileData},
	prelude::*,
};

use crate::{
	contexts::DragContext,
	rpc::{
		aoba::{self, FileInfo, IdList, UploadRequest, UploadResult, UploadTarget, upload_target_response},
		get_rpc_client,
	},
};

#[derive(PartialEq, Clone, Props)]
pub struct UploadAreaProps
{
	children: Element,
	on_upload_complete: Callback,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum UploadState
{
	Idle,
	Uploading(usize),
	Done,
}

#[component]
pub fn UploadArea(props: UploadAreaProps) -> Element
{
	let mut drag_context = use_context::<DragContext>();
	let mut upload_state = use_signal(|| UploadState::Idle);
	let mut file_count = use_signal(|| None::<usize>);
	let mut upload_progress = use_signal(|| 0.0_f32);

	let on_files_dropped = move |e: Event<DragData>| {
		e.prevent_default();
		info!("Drop");
		drag_context.is_dragging.set(false);
		file_count.set(Some(e.files().len()));
		let total_file_size: u64 = e.files().iter().map(|f| f.size()).sum();

		upload_state.set(UploadState::Uploading(0));
		spawn(async move {
			match upload_files(e.files(), upload_state).await
			{
				Ok(_) =>
				{
					props.on_upload_complete.call(());
					upload_state.set(UploadState::Done);
				}
				Err(err_msg) => warn!("Upload failed: {:?}", err_msg.join(", ")),
			};
		});
	};

	let is_dragging = use_memo(move || match drag_context.is_dragging.cloned()
	{
		true => "dragging",
		false => "",
	});
	rsx! {
		div{
			id: "uploadArea",
			class: is_dragging(),
			ondrop: on_files_dropped,
			UploadStatus { status: upload_state.cloned() }
			UploaderOverlay {
				{props.children}
			 }
		}
	}
}

#[component]
fn UploadStatus(status: UploadState) -> Element
{
	rsx! {}
}

async fn upload_files(files: Vec<FileData>, upload_state: Signal<UploadState>) -> Result<UploadResult, Vec<String>>
{
	let upload_request = UploadRequest {
		files: files
			.iter()
			.map(|f| FileInfo {
				filename: f.name(),
				size: f.size(),
			})
			.collect(),
	};
	let mut aoba_client = get_rpc_client();
	let result = aoba_client.start_upload(upload_request).await;
	if let Ok(response) = result
		&& let Some(targets) = response.into_inner().result
	{
		info!("Starting upload");
		match targets
		{
			upload_target_response::Result::Targets(upload_targets) =>
			{
				let client = reqwest::Client::new();
				let mut uploaded_ids = Vec::with_capacity(upload_targets.targets.len());
				for file_tgt in upload_targets.targets
				{
					let file = files.iter().find(|f| f.name() == file_tgt.filename);
					match file
					{
						Some(file) =>
						{
							if let Some(id) = upload_file(&client, file, file_tgt).await
							{
								uploaded_ids.push(id);
							}
						}
						_ => warn!("No matching file for upload: {:?}", file_tgt.filename),
					}
				}
				return if !uploaded_ids.is_empty()
				{
					match aoba_client.complete_upload(IdList { value: uploaded_ids }).await
					{
						Ok(upload_result) => Ok(upload_result.into_inner()),
						Err(status) => Err(vec![format!("Failed to complete upload: {}", status).to_string()]),
					}
				}
				else
				{
					Err(vec!["No files uploaded".to_string()])
				};
			}
			upload_target_response::Result::Error(_) => todo!("Handle target creation error"),
		}
	}
	else
	{
		Err(vec!["No upload".to_string()])
	}
}

async fn upload_file(client: &reqwest::Client, file: &FileData, file_tgt: UploadTarget) -> Option<aoba::Id>
{
	match file.read_bytes().await
	{
		Ok(bytes) =>
		{
			info!("Uploading file: {}", file_tgt.filename);
			let upload_result = client
				.put(file_tgt.signed_url)
				.header("Content-Length", bytes.len())
				.header("Content-Type", file_tgt.content_type)
				.body(bytes)
				.send()
				.await;
			match upload_result
			{
				Ok(_upload) =>
				{
					info!("File upload complete: {}", file_tgt.filename);
					file_tgt.id
				}
				Err(err) =>
				{
					warn!("Failed to upload file: {:?}", err);
					None
				}
			}
		}
		Err(file_read_err) =>
		{
			warn!("Failed to read file: {:?}", file_read_err);
			None
		}
	}
}

#[component]
pub fn UploaderOverlay(children: Element) -> Element
{
	let mut ctx = use_context::<DragContext>();
	let on_drag_exit = move |_e: Event<DragData>| {
		ctx.is_dragging.set(false);
	};
	rsx! {
		div{
			id: "uploadOverlay",
			ondragexit: on_drag_exit,
			ondragend: on_drag_exit,
			ondragleave: on_drag_exit,
			div{
				class: "display",
				{children}
			}
			input{
				type: "file",
				multiple: true
			}
		}
	}
}

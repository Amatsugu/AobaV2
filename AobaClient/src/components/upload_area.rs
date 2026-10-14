use dioxus::{html::HasFileData, prelude::*};

use crate::{
	contexts::DragContext,
	file_uploader::{UploadState, upload_files},
};

#[derive(PartialEq, Clone, Props)]
pub struct UploadAreaProps
{
	children: Element,
	on_upload_complete: Callback,
}

#[component]
pub fn UploadArea(props: UploadAreaProps) -> Element
{
	let mut drag_context = use_context::<DragContext>();
	let mut upload_state = use_signal(|| UploadState::Idle);
	let mut file_count = use_signal(|| None::<usize>);
	let _upload_progress = use_signal(|| 0.0_f32);

	let on_files_dropped = move |e: Event<DragData>| {
		e.prevent_default();

		if e.files().is_empty()
		{
			drag_context.is_dragging.set(false);
			return;
		}
		drag_context.is_dragging.set(false);
		file_count.set(Some(e.files().len()));
		let _total_file_size: u64 = e.files().iter().map(|f| f.size()).sum();

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
			onmouseup: move |_|{ drag_context.is_dragging.set(false); },
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

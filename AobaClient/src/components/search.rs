use dioxus::prelude::*;

use crate::{
	components::icons::UpCloud,
	file_uploader::{UploadState, upload_files},
};

#[component]
pub fn Search(
	query: String,
	oninput: Option<EventHandler<String>>,
	onchange: Option<EventHandler<String>>,
	on_upload_complete: EventHandler<()>,
) -> Element
{
	let mut upload_state = use_signal(|| UploadState::Idle);
	rsx! {
		div { class: "searchBar",
			input {
				r#type: "search",
				placeholder: "Search Files",
				value: query,
				oninput: move |event| {
					if let Some(handler) = oninput {
						handler.call(event.value());
					}
				},
				onchange: move |event|{
					if let Some(handler) = onchange {
						handler.call(event.value());
					}
				}
			}
			div {
				class: "uploadBtn",
				input {
					type: "file",
					multiple: true,
					oninput: move |e|{
						if let UploadState::Uploading(_) = upload_state.cloned() {
							return;
						}
						let files = e.files();
						spawn(async move{
							match upload_files(files, upload_state).await
							{
								Ok(_) =>
								{
									on_upload_complete.call(());
									upload_state.set(UploadState::Done);
								}
								Err(err_msg) => warn!("Upload failed: {:?}", err_msg.join(", ")),
							};
						});
					}
				}
				UpCloud{}
			}
		}
	}
}

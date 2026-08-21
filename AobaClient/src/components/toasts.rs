use dioxus::prelude::*;
use futures_timer::Delay;
use futures_util::stream::{FuturesUnordered, StreamExt};

use crate::{
	components::icons::Cross,
	models::toasts::{ToastCommand, ToastEntry, ToastsContext},
};

const TOASTS_CSS: Asset = asset!("/assets/style/toasts.scss");
#[component]
pub fn ToastsDisplay() -> Element
{
	let ctx = use_context::<ToastsContext>();
	rsx! {
		document::Link { rel: "stylesheet", href: TOASTS_CSS }
		div{
			id: "toastDisplay",
			for toast in ctx.toasts.cloned() {
				Toast {
					key: "{toast.id}",
					toast
				}
			}
		}
	}
}

#[component]
pub fn Toast(toast: ToastEntry) -> Element
{
	let ctx = use_context::<ToastsContext>();
	let slide_out: String = if toast.is_dismissing
	{
		"animation-name: toastSlideOut;".into()
	}
	else
	{
		"animation-name: toastSlideIn;".into()
	};

	rsx! {
		div {
			class: "toast {toast.level.as_class()}",
			style: slide_out,
			onanimationend: move |e: Event<AnimationData>|{
				if toast.is_dismissing && e.data().animation_name() == "toastSlideOut"{
					ctx.handle.send(ToastCommand::Remove(toast.id));
				}
			},
			div {
				class: "title",
				{toast.title}
			}
			if let Some(message) = toast.message{
				div {
					class: "message",
					{message}
				}
			}
			div {
				class: "dismiss",
				onclick: move |_| ctx.handle.send(ToastCommand::Dismiss(toast.id)),
				Cross {}
			}
			if let Some(duration) = toast.duration {
				div {
					class: "progress",
					div { class: "fill", style: "animation-duration: {duration.as_secs_f32()}s" }
				}
			}
		}
	}
}

pub fn init_toasts() -> ToastsContext
{
	let toasts = use_signal(Vec::<ToastEntry>::new);
	let handle = use_coroutine(move |mut rx: UnboundedReceiver<ToastCommand>| {
		let mut toasts = toasts;
		async move {
			let mut next_id = 0_usize;
			let mut timers = FuturesUnordered::new();
			loop
			{
				futures_util::select! {
					cmd = rx.next() =>{
						let Some(cmd) = cmd else {break};
						match cmd{
							ToastCommand::Push { title, message, level, duration } =>{
								let id = next_id;
								next_id += 1;
								toasts.write().push(ToastEntry { id, title, message, level, ..Default::default() });

								if let Some(duration) = duration{
									timers.push(async move {
										Delay::new(duration).await;
										id
									});
								}
							}
							ToastCommand::Dismiss(id) => {
								if let Some(toast) = toasts.write().iter_mut().find(|t| t.id == id){
									toast.is_dismissing = true;
								}
							}
							ToastCommand::Remove(id) => {
								toasts.write().retain(|t| t.id != id);
							}
						}
					}
					expired_id = timers.select_next_some() =>{
						if let Some(toast) = toasts.write().iter_mut().find(|t| t.id == expired_id){
							toast.is_dismissing = true;
						}
					}
				}
			}
		}
	});
	ToastsContext { handle, toasts }
}

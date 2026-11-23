use std::time::Duration;

use dioxus::prelude::*;
use futures_timer::Delay;
use futures_util::{StreamExt, stream::FuturesUnordered};

use crate::rpc::{
	AuthorizedClient,
	aoba::{Dimensions, Id, Thumb, ThumbnailRequest, ThumbnailSize, aoba_rpc_client::AobaRpcClient},
	get_rpc_client,
};

#[derive(PartialEq)]
pub struct ThumbnailWait
{
	pub id: String,
	pub size: ThumbnailSize,
}

pub struct ThumbnailResult
{
	pub id: String,
	pub result: Option<(String, Dimensions)>,
}

#[derive(Clone)]
pub struct ThumbnailLoaderContext
{
	pub waitlist: Signal<Vec<ThumbnailWait>>,
	pub results: Signal<Vec<ThumbnailResult>>,
	pub handle: Coroutine<ThumbnailWait>,
}

pub fn init_thumbnail_loader() -> ThumbnailLoaderContext
{
	let ctx = create_thumbnail_loader();
	use_context_provider(|| ctx)
}
fn create_thumbnail_loader() -> ThumbnailLoaderContext
{
	let waitlist = use_signal(Vec::<ThumbnailWait>::new);
	let results = use_signal(Vec::<ThumbnailResult>::new);
	let handle = use_coroutine(move |mut rx: UnboundedReceiver<ThumbnailWait>| {
		let waitlist = waitlist;
		let results = results;
		async move {
			let mut timers = FuturesUnordered::new();
			loop
			{
				let mut rpc = get_rpc_client();

				futures_util::select! {
					req = rx.next() => {
						if let Some(req) = req {
							add_thumbnail_listener(req, waitlist);
							if timers.is_empty()
							{
								timers.push(delay());
							}
						}
					}
					_ = timers.select_next_some() => {
						check_status(&mut rpc, waitlist, results).await;
						if !waitlist.read().is_empty()
						{
							timers.push(delay());
						}
					}
				}
			}
		}
	});

	return ThumbnailLoaderContext {
		waitlist,
		handle,
		results,
	};
}

async fn delay() -> Option<()>
{
	Delay::new(Duration::from_secs(1)).await;
	Some(())
}

fn add_thumbnail_listener(req: ThumbnailWait, mut waitlist: Signal<Vec<ThumbnailWait>>)
{
	let mut waitlist = waitlist.write();
	if waitlist.contains(&req)
	{
		return;
	}
	else
	{
		waitlist.push(req);
	}
}

async fn check_status(
	rpc: &mut AobaRpcClient<AuthorizedClient>,
	mut waitlist: Signal<Vec<ThumbnailWait>>,
	mut results: Signal<Vec<ThumbnailResult>>,
)
{
	let Ok(res) = rpc
		.get_thumbnail(get_request_body(waitlist))
		.await
		.map(|r| r.into_inner().info)
	else
	{
		return;
	};
	for status in res
	{
		if status.generating
		{
			continue;
		}
		if let Some(id) = status.id
		{
			info!("url: {:?} - size: {:?}", status.url.is_some(), status.size.is_some());
			if let Some(url) = status.url
				&& let Some(size) = status.size
				&& !status.generating
			{
				info!("Thumbnail Result");
				results.write().push(ThumbnailResult {
					id: id.value.clone(),
					result: Some((url, size)),
				});
				waitlist.write().retain(|r| r.id != id.value);
			}
			else if !status.generating
			{
				waitlist.write().retain(|r| r.id != id.value);
			}
		}
	}
}

fn get_request_body(waitlist: Signal<Vec<ThumbnailWait>>) -> ThumbnailRequest
{
	ThumbnailRequest {
		thumbs: waitlist
			.read()
			.iter()
			.map(|r| Thumb {
				media_id: Some(Id { value: r.id.clone() }),
				size: r.size.into(),
			})
			.collect(),
	}
}

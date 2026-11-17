use dioxus::logger::tracing::info;
use futures_util::future::LocalBoxFuture;
use tower_layer::Layer;
use tower_service::Service;

#[derive(Clone)]
pub struct UnAuthLayer
{
	pub unauth: fn(),
	pub refresh: fn(String, u64),
}

impl<S> Layer<S> for UnAuthLayer
{
	type Service = UnauthService<S>;

	fn layer(&self, inner: S) -> Self::Service
	{
		UnauthService {
			inner,
			unauth: self.unauth,
			refresh: self.refresh,
		}
	}
}

#[derive(Clone)]
pub struct UnauthService<S>
{
	inner: S,
	unauth: fn(),
	refresh: fn(String, u64),
}

impl<S, B, RB> Service<http::Request<B>> for UnauthService<S>
where
	S: Service<http::Request<B>, Response = http::Response<RB>> + Clone + 'static,
	S::Future: 'static,
	// S::Error: std::error::Error + 'static,
	B: 'static,
{
	type Response = S::Response;

	type Error = S::Error;

	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

	fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>>
	{
		self.inner.poll_ready(cx)
	}

	fn call(&mut self, req: http::Request<B>) -> Self::Future
	{
		let clone = self.inner.clone();
		let mut inner = std::mem::replace(&mut self.inner, clone);
		let unauth = self.unauth.clone();
		let refresh = self.refresh.clone();
		Box::pin(async move {
			match inner.call(req).await
			{
				Ok(res) =>
				{
					let http_401 = res.status() == http::StatusCode::UNAUTHORIZED;
					if let Some(token_refresh) = res.headers().get("token-refresh").and_then(|v| v.to_str().ok())
						&& let Some(token_exp) = res.headers().get("token-refresh-exp").and_then(|v| v.to_str().ok())
						&& let Ok(token_exp) = token_exp.parse::<u64>()
					{
						refresh(token_refresh.to_string(), token_exp);
					}
					let grpc_16 = res.headers().get("grpc-status").and_then(|v| v.to_str().ok()) == Some("16");
					if http_401 || grpc_16
					{
						unauth();
						info!("UnauthLayer: logout");
					}
					Ok(res)
				}
				Err(err) =>
				{
					// if let Some(e) = (&err as &dyn std::error::Error).downcast_ref::<tonic_web_wasm_client::Error>()
					// {
					// 	match e
					// 	{
					// 		tonic_web_wasm_client::Error::MissingContentTypeHeader => unauth(),
					// 		_ => (),
					// 	}
					// }
					unauth();
					Err(err)
				}
			}
		})
	}
}

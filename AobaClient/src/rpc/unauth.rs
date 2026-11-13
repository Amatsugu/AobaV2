use futures_util::future::{BoxFuture, LocalBoxFuture};
use tower_layer::Layer;
use tower_service::Service;

#[derive(Clone)]
pub struct UnAuthLayer(pub fn());

impl<S> Layer<S> for UnAuthLayer {
	type Service = UnauthService<S>;

	fn layer(&self, inner: S) -> Self::Service {
		UnauthService { inner, cb: self.0 }
	}
}

#[derive(Clone)]
pub struct UnauthService<S> {
	inner: S,
	cb: fn(),
}

impl<S, B, RB> Service<http::Request<B>> for UnauthService<S>
where
	S: Service<http::Request<B>, Response = http::Response<RB>> + Clone + 'static,
	S::Future: 'static,
	B: 'static,
{
	type Response = S::Response;

	type Error = S::Error;

	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

	fn poll_ready(
		&mut self,
		cx: &mut std::task::Context<'_>,
	) -> std::task::Poll<Result<(), Self::Error>> {
		self.inner.poll_ready(cx)
	}

	fn call(&mut self, req: http::Request<B>) -> Self::Future {
		let clone = self.inner.clone();
		let mut inner = std::mem::replace(&mut self.inner, clone);
		let cb = self.cb.clone();
		Box::pin(async move {
			let res = inner.call(req).await?;
			let http_401 = res.status() == http::StatusCode::UNAUTHORIZED;
			let grpc_16 = res
				.headers()
				.get("grpc-status")
				.and_then(|v| v.to_str().ok())
				== Some("16");
			if http_401 || grpc_16 {
				cb();
			}
			Ok(res)
		})
	}
}

use dioxus::signals::Signal;

#[derive(Clone, Copy, Default)]
pub struct AuthContext {
	pub jwt: Signal<Option<String>>,
}

use crate::{
	layouts::MainLayout,
	views::{Home, HomePaged, Media, Settings},
};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
	#[layout(MainLayout)]

		#[route("/")]
		Home { },
		#[route("/?:page&:q")]
		HomePaged { page: i32, q: String },
		#[route("/media/:id")]
		Media { id: String },
		#[route("/settings")]
		Settings {},
	// #[end_layout]
}

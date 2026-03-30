use crate::{
	layouts::MainLayout,
	views::{Home, Media, Settings},
};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
	#[layout(MainLayout)]

		#[route("/?:page&:q")]
		Home { page: Option<i32>, q: Option<String> },
		// #[route("/")]
		// Home { },
		#[route("/media/:id")]
		Media { id: String },
		#[route("/settings")]
		Settings {},
	// #[end_layout]
}

use crate::{
	layouts::MainLayout,
	views::{Home, Media, Settings},
};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
	#[layout(MainLayout)]

		#[route("/")]
		Home { },
		#[route("/media/:id")]
		Media { id: String },
		#[route("/settings")]
		Settings {},
	// #[end_layout]
}

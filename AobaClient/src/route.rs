use crate::{
	layouts::MainLayout,
	views::{Home, Settings},
};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
	#[layout(MainLayout)]
		#[route("/")]
		Home {},
		#[route("/settings")]
		Settings {},
	// #[end_layout]
}

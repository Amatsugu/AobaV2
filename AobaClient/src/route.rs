use crate::{
	layouts::{BasicLayout, MainLayout},
	views::{Home, Login, Settings},
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
	#[end_layout]
	#[layout(BasicLayout)]
		#[route("/login")]
		Login {},
}

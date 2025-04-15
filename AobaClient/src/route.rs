use dioxus::prelude::*;
use crate::views::{Home, Settings};
use crate::components::MainLayout;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
	#[layout(MainLayout)]
	#[route("/")]
	Home {},
	#[route("/settings")]
	Settings {},
}

use std::time::Duration;

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Copy)]
pub struct ToastsContext
{
	pub toasts: Signal<Vec<ToastEntry>>,
	pub handle: Coroutine<ToastCommand>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToastCommand
{
	Push
	{
		title: String,
		message: Option<String>,
		level: ToastLevel,
		duration: Option<Duration>,
	},
	Dismiss(usize),
	Remove(usize),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ToastEntry
{
	pub id: usize,
	pub title: String,
	pub message: Option<String>,
	pub level: ToastLevel,
	pub is_dismissing: bool,
	pub duration: Option<Duration>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ToastLevel
{
	#[default]
	Info,
	Warning,
	Error,
}

impl ToastLevel
{
	pub fn as_class(&self) -> String
	{
		match self
		{
			ToastLevel::Info => "info",
			ToastLevel::Warning => "warn",
			ToastLevel::Error => "error",
		}
		.into()
	}
}

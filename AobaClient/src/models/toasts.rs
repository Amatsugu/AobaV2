use std::time::Duration;

use dioxus::prelude::*;

#[derive(Clone, PartialEq, Copy)]
pub struct ToastsContext
{
	pub toasts: Signal<Vec<ToastEntry>>,
	pub handle: Coroutine<ToastCommand>,
}

impl ToastsContext
{
	pub fn push(&self, cmd: ToastCommand)
	{
		self.handle.send(cmd);
	}
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

impl ToastCommand
{
	pub fn push_info(title: impl Into<String>) -> ToastCommand
	{
		ToastCommand::Push {
			title: title.into(),
			message: None,
			level: ToastLevel::Info,
			duration: None,
		}
	}

	pub fn push_info_with_message(title: impl Into<String>, message: impl Into<String>) -> ToastCommand
	{
		ToastCommand::Push {
			title: title.into(),
			message: Some(message.into()),
			level: ToastLevel::Info,
			duration: None,
		}
	}

	pub fn push_warning(title: impl Into<String>) -> ToastCommand
	{
		ToastCommand::Push {
			title: title.into(),
			message: None,
			level: ToastLevel::Warning,
			duration: None,
		}
	}

	pub fn push_warning_with_message(title: impl Into<String>, message: impl Into<String>) -> ToastCommand
	{
		ToastCommand::Push {
			title: title.into(),
			message: Some(message.into()),
			level: ToastLevel::Warning,
			duration: None,
		}
	}

	pub fn push_error(title: impl Into<String>) -> ToastCommand
	{
		ToastCommand::Push {
			title: title.into(),
			message: None,
			level: ToastLevel::Error,
			duration: None,
		}
	}

	pub fn push_error_with_message(title: impl Into<String>, message: impl Into<String>) -> ToastCommand
	{
		ToastCommand::Push {
			title: title.into(),
			message: Some(message.into()),
			level: ToastLevel::Error,
			duration: None,
		}
	}

	pub fn with_duration(mut self, duration: Duration) -> Self
	{
		if let ToastCommand::Push { duration: d, .. } = &mut self
		{
			*d = Some(duration);
		}
		self
	}

	pub fn with_message(mut self, message: String) -> Self
	{
		if let ToastCommand::Push { message: m, .. } = &mut self
		{
			*m = Some(message);
		}
		self
	}
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

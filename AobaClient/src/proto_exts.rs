use std::fmt::Display;

use crate::rpc::aoba::{Dimensions, MediaClass, MediaModel, MediaType, ThumbnailSize};

impl MediaModel
{
	pub fn get_thumbnail_url(&self, size: ThumbnailSize) -> Option<String>
	{
		self.thumbnails
			.iter()
			.find_map(|t| if t.size() == size { Some(t.url.clone()) } else { None })
	}
}

impl Display for MediaClass
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		let v = match self
		{
			MediaClass::Unspecified => "Unkown",
			MediaClass::Standard => "Standard",
			MediaClass::Nsfw => "NSFW",
			MediaClass::Secret => "Secret",
		};
		f.write_str(v)
	}
}

impl Display for MediaType
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		let v = match self
		{
			MediaType::Image => "Image",
			MediaType::Audio => "Audio",
			MediaType::Video => "Video",
			MediaType::Text => "Text",
			MediaType::Code => "Code",
			MediaType::Raw => "Raw",
			_ => "Unknown",
		};
		f.write_str(v)
	}
}

impl MediaClass
{
	pub fn to_class_name(&self) -> &str
	{
		match self
		{
			MediaClass::Nsfw => "blur",
			MediaClass::Secret => "secret",
			_ => "",
		}
	}
}

impl Display for Dimensions
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		f.write_fmt(format_args!("{}x{}", self.height, self.width))
	}
}

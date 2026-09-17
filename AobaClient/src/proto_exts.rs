use crate::rpc::aoba::{MediaClass, MediaModel, MediaType, ThumbnailSize};

impl MediaModel {
	pub fn get_thumbnail_url(&self, size: ThumbnailSize) -> Option<String> {
		self.thumbnails.iter().find_map(|t| {
			if t.size() == size {
				Some(t.url.clone())
			} else {
				None
			}
		})
	}
}

impl From<MediaClass> for &str {
	fn from(value: MediaClass) -> Self {
		match value {
			MediaClass::Unspecified => "Unkown",
			MediaClass::Standard => "Standard",
			MediaClass::Nsfw => "NSFW",
			MediaClass::Secret => "Secret",
		}
	}
}

impl From<MediaType> for &str {
	fn from(value: MediaType) -> Self {
		match value {
			MediaType::Image => "Image",
			MediaType::Audio => "Audio",
			MediaType::Video => "Video",
			MediaType::Text => "Text",
			MediaType::Code => "Code",
			MediaType::Raw => "Raw",
			_ => "Unknown",
		}
	}
}

impl MediaClass {
	pub fn to_class_name(&self) -> &str {
		match self {
			MediaClass::Nsfw => "blur",
			MediaClass::Secret => "secret",
			_ => "",
		}
	}
}

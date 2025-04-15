use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Media {
	pub id: String,
	pub media_id: String,
	pub filename: String,
	pub media_type: MediaType,
	pub ext: String,
	pub view_count: i32,
	pub owner: String,
}
#[derive(Serialize_repr, Deserialize_repr, Clone, PartialEq)]
#[repr(i32)]
pub enum MediaType {
	Image,
	Audio,
	Video,
	Text,
	Code,
	Raw,
}

use std::error::Error;
use serde::{Serialize, Deserialize};

use crate::{INPUT_DIR, utils};

//
// Structs for JSON parsing
//

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
	pub creators: Vec<Creator>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Creator {
	pub name: String,
	pub display_name: String,
	pub categories: Vec<Category>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
	pub title: String,
	pub addons: Vec<String>,

}

//
// Gets data from data.json
//

pub fn get_data() -> Result<Data, Box<dyn Error>> {
	Ok(
		serde_json::from_str(
			utils::file_path_to_string(
				format!("{}/data.json", INPUT_DIR).as_str()
			)?.as_str()
		)?
	)
}

//
// Structs for storing thumbnail information
//

#[derive(Debug, Serialize, Deserialize)]
pub struct ThumbnailCreator {
	pub name: String,
	pub display_name: String,
	pub categories: Vec<ThumbnailCategory>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ThumbnailCategory {
	pub title: String,
	pub addons: Vec<ThumbnailAddon>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ThumbnailAddon {
	pub name: String,
	pub has_thumbnail: bool,
}

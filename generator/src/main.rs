use std::{fs, io};
use std::env::set_current_dir;
use std::error::Error;
use std::path::Path;

mod data;
mod utils;

mod gen_css;
mod gen_html;
mod gen_images;

pub const INPUT_DIR : &str = "gen-data";
pub const OUTPUT_DIR: &str = "public";

fn main() -> Result<(), Box<dyn Error>> {

	let start;
	let elapsed;

	start = std::time::Instant::now();

	set_current_dir("../")?;

	// Get data from data.json
	let data = data::get_data()?;

	// Check for --dir flag
	let args = std::env::args().collect::<Vec<String>>();

	if args.len() == 2 && args[1].as_str() == "--dir" {
		create_dirs(&data)?;
	} else {

		// Main stuff
		gen_css::gen_css()?;
		gen_html::gen_html(&data)?;
		gen_images::gen_images()?;

		copy_file("robots.txt")?;
		copy_file("favicon.ico")?;
		copy_file("static/no_image.jpg")?;
	}

	elapsed = start.elapsed();

	println!("Time taken: {:?}", elapsed);

	Ok(())
}

//
// Creates dirs for putting files in OUTPUT_DIR
//

fn create_dirs(data: &data::Data) -> Result<(), io::Error> {

	let files_dir = Path::new(&format!("{}/files/", OUTPUT_DIR)).to_path_buf();

	for creator in &data.creators {

		let mut creator_dir = files_dir.clone();
		creator_dir.push(&creator.name);

		if !creator_dir.exists() {
			fs::create_dir(creator_dir)?;
		}
	}

	Ok(())
}

//
// Copies a file from INPUT_DIR to OUTPUT_DIR
//

fn copy_file(relative_file_path: &str) -> Result<(), io::Error> {

	let input_path: String = format!("{}/{}", INPUT_DIR, relative_file_path);
	let input_path = Path::new(&input_path);
	let output_path: String = format!("{}/{}", OUTPUT_DIR, relative_file_path);
	let output_path = Path::new(&output_path);

	fs::create_dir_all(output_path.parent().unwrap())?;

	fs::copy(input_path, output_path)?;

	Ok(())
}
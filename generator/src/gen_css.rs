use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use const_format::concatcp;

use crate::utils;
use crate::{OUTPUT_DIR, INPUT_DIR};

const CSS_GLOB  : &str = concatcp!(INPUT_DIR, "/static/*.css");
const CSS_OUTDIR: &str = concatcp!(OUTPUT_DIR, "/static/");

pub fn gen_css() -> Result<(), Box<dyn Error>> {

	// Create parent dir
	let parent_dir = Path::new(CSS_OUTDIR);
	fs::create_dir_all(parent_dir)?;

	// Get css files
	for file in glob::glob(CSS_GLOB)? {

		let file = file?;
		let file_name = file.file_name().unwrap().to_str().unwrap();
		let mut output_file = Path::new(CSS_OUTDIR).to_path_buf();
		output_file.push(file_name);

		let css = utils::file_path_to_string(file.to_str().unwrap())?;

		let css = html_minifier::css::minify(css.as_str())?;

		let mut file = File::options().create(true).write(true).open(output_file)?;

		file.write(css.as_bytes())?;

		file.flush()?;
	}

	Ok(())
}
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use const_format::concatcp;
use image::DynamicImage;
use image::imageops::FilterType;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use image::io::Reader as ImageReader;

use crate::{OUTPUT_DIR, INPUT_DIR};

const RESOLUTION_MULTIPLIER: usize = 39;
const OUTPUT_WIDTH         : usize = RESOLUTION_MULTIPLIER * 16;
const OUTPUT_HEIGHT        : usize = RESOLUTION_MULTIPLIER * 9;
const QUALITY              : usize = 25;

pub fn gen_images() -> Result<(), Box<dyn Error>> {

	let input_files = get_input_files()?;

	input_files.par_iter()
		.for_each(|input_file| {

			let creator_dir = Path::new(input_file);
			let creator_dir = creator_dir.parent().unwrap().file_name().unwrap().to_str().unwrap();

			let output_file = Path::new(input_file);
			let output_file = output_file.file_stem().unwrap().to_str().unwrap();

			let input_file = Path::new(input_file).file_name().unwrap().to_str().unwrap();

			compress_and_save(
				&format!("{}/thumbnails/{}/{}", INPUT_DIR, creator_dir, input_file),
				&format!("{}/thumbnails/{}/{}.jpg", OUTPUT_DIR, creator_dir, output_file),
			).unwrap();
		});

	Ok(())
}

const THUMBNAIL_GLOB: &str = concatcp!(INPUT_DIR, "/thumbnails/*/*.*");

fn get_input_files() -> Result<Vec<String>, Box<dyn Error>> {

	let mut input_files = Vec::new();

	for file in glob::glob(THUMBNAIL_GLOB)? {

		let mut file = file?;

		let file_name = String::from(file.file_name().unwrap().to_str().unwrap());

		file.pop();

		let dir_name = String::from(file.file_name().unwrap().to_str().unwrap());

		let file_path = format!("{}/{}", dir_name, file_name);

		input_files.push(file_path);

	}

	Ok(input_files)
}
fn compress_and_save(input_path: &String, output_path: &String) -> Result<(), Box<dyn Error>> {

	// Get data form input image
	let img = ImageReader::open(input_path)?.decode()?;
	let img = DynamicImage::from(img.into_rgb8());
	let img = img.resize_exact(OUTPUT_WIDTH as u32, OUTPUT_HEIGHT as u32, FilterType::Lanczos3);
	let pixels = img.as_bytes();

	// Compression
	let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);

	comp.set_size(OUTPUT_WIDTH, OUTPUT_HEIGHT);
	comp.set_mem_dest();
	comp.set_quality(QUALITY as f32);
	comp.set_smoothing_factor(64);

	comp.start_compress();

	assert!(comp.write_scanlines(&pixels[..]));

	comp.finish_compress();

	// Save image
	let output_path = Path::new(output_path);
	fs::create_dir_all(output_path.parent().unwrap())?;

	let jpeg_bytes = comp.data_to_vec().unwrap();

	let mut output_file = File::options().create(true).write(true).open(output_path)?;
	output_file.write_all(&jpeg_bytes)?;
	output_file.flush()?;

	Ok(())
}
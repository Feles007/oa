use std::fs::File;
use std::io;
use std::io::Read;

pub fn file_path_to_string(path: &str) -> Result<String, io::Error> {

	let mut buffer = String::new();

	let mut file = File::open(path)?;

	file.read_to_string(&mut buffer)?;

	Ok(buffer)
}
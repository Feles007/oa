use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use const_format::concatcp;
use html_minifier::HTMLMinifier;
use serde::Serialize;
use tera::{Context, Tera};

use crate::data::{Creator, Data, ThumbnailAddon, ThumbnailCategory, ThumbnailCreator};
use crate::{OUTPUT_DIR, INPUT_DIR};

const TEMPLATE_GLOB: &str = concatcp!(INPUT_DIR, "/templates/*.html");

pub fn gen_html(data: &Data) -> Result<(), Box<dyn Error>> {

	// Get templates
	let tera = Tera::new(TEMPLATE_GLOB)?;

	// Render and save templates
	render_and_save("index.html", concatcp!(OUTPUT_DIR, "/index.html"), &tera, data)?;
	render_and_save("error.html", concatcp!(OUTPUT_DIR, "/error/index.html"), &tera, data)?;
	render_and_save("all.html", concatcp!(OUTPUT_DIR, "/all/index.html"), &tera, data)?;

	// Render and save creator templates
	for creator in &data.creators {

		// This is a real mess but it converts the Creator struct to
		// a ThumbnailCreator to have the has_thumbnail value inside
		// the template

		let creator = creator_to_thumbnail_creator(&creator);

		// Get path of output file
		let mut output_file = String::from(OUTPUT_DIR);
		output_file.push_str(format!("/creator/{}/index.html", creator.name).as_str());
		let output_file = output_file.as_str();

		render_and_save("creator.html", output_file, &tera, &creator)?;
	}

	Ok(())
}
fn render_and_save(template_name: &str, save_path: &str, tera: &Tera, data: &impl Serialize) -> Result<(), Box<dyn Error>> {

	// Render template
	let rendered = tera.render(template_name, &Context::from_serialize(data)?)?;

	// Minify HTML
	let mut html_minifier = HTMLMinifier::new();
	html_minifier.set_remove_comments(true);
	html_minifier.set_minify_code(true);

	html_minifier.digest(rendered)?;

	// Save file
	let save_path = Path::new(save_path);

	fs::create_dir_all(save_path.parent().unwrap())?;

	let mut file = File::options().create(true).write(true).open(save_path)?;

	file.write_all(&html_minifier.get_html())?;

	file.flush()?;

	Ok(())
}
fn creator_to_thumbnail_creator(creator: &Creator) -> ThumbnailCreator {

	let mut thumbnail_creator = ThumbnailCreator {
		name: creator.name.clone(),
		display_name: creator.display_name.clone(),
		categories: Vec::new(),
	};

	for category in &creator.categories {

		let addons = category.addons.clone();
		let mut has_thumbnail = Vec::with_capacity(addons.len());

		for addon in &addons {
			has_thumbnail.push(addon_has_thumbnail(&creator.name, addon));
		}

		let mut thumbnail_addons = Vec::new();

		for i in 0..addons.len() {
			thumbnail_addons.push(ThumbnailAddon {
				name: addons[i].clone(),
				has_thumbnail: has_thumbnail[i],
			});
		}

		thumbnail_creator.categories.push(ThumbnailCategory {
			title: category.title.clone(),
			addons: thumbnail_addons,
		});
	}

	thumbnail_creator
}
fn addon_has_thumbnail(creator_name: &String, addon_name: &String) -> bool {

	let thumbnail_glob = format!("{}/thumbnails/{}/{}.*", INPUT_DIR, creator_name, addon_name);

	let search = glob::glob(thumbnail_glob.as_str());

	let search = match search {
		Ok(search) => search,
		Err(_) => return false,
	};

	let count = search.count();

	return if count == 1 {
		true
	} else {
		false
	};
}
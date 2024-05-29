mod parse;
mod md_file;
mod traits;

use std::fs;
use std::path::PathBuf;

fn main() {
	let filelist = fs::read_dir("vault").unwrap();

	for file in filelist {

		let file_path = file.unwrap().path();
		println!("Testing file: {:?}", file_path.clone());
		let file_contents = fs::read_to_string(&file_path).unwrap();
		let md_file = md_file::MDFile::new(file_path.clone());
		let result = md_file.to_string();
		let expected = file_contents;
		assert_eq!(result, expected, "Failed on file: {:?}", file_path);
	}
	println!("All stability tests passed!")
}
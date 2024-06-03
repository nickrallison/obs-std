use std::path::PathBuf;

use crate::vault::Vault;

mod parse;
mod md_file;
mod vault;

fn main() {
	let start_time = std::time::Instant::now();
	let path = PathBuf::from("vault");
	let mut vault = Vault::new(path);
	vault.link_all_files();
	let md_file_path = PathBuf::from("vault/Natural Deduction.md");
	let md_file = vault.get_md_file(&md_file_path).unwrap();
	println!("MD File: {}", md_file);
	println!("Time: {:?}", start_time.elapsed());
}
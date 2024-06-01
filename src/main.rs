use std::path::PathBuf;
use crate::md_file::MDFile;
use crate::parse::Line;

use crate::vault::Vault;

mod parse;
mod md_file;
mod traits;
mod vault;

fn main() {
	let path = PathBuf::from("vault");
	let vault = Vault::new(path);
	let md_file_path = PathBuf::from("vault/Natural Deduction.md");
	let md_file: &MDFile = vault.get_md_file(&md_file_path).unwrap();
	let strings: Vec<&str> = vec!["modus", "ponens"];
	println!("MD File: {:?}", md_file.get_title());
	let links = vault.get_outgoing_links(md_file);
	println!("Links: {:?}", links);
}
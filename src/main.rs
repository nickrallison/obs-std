use std::path::PathBuf;
use crate::md_file::MDFile;

use crate::vault::Vault;

mod parse;
mod md_file;
mod traits;
mod vault;

fn main() {
	let path = PathBuf::from("vault");
	let vault = Vault::new(path);

	let md_file: &MDFile = vault.random_md_file();
	let outgoint_links = vault.get_outgoing_links(md_file);
	println!("Outgoing links: {:?}", outgoint_links);
}
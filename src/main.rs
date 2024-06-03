use std::path::PathBuf;
use crate::options::LinkerOptions;

use crate::vault::Vault;

mod parse;
mod md_file;
mod vault;
mod linking;
mod options;

fn main() {
	let start_time = std::time::Instant::now();
	let path = PathBuf::from(r"C:\Users\nickr\Documents\Vault\500-Zettelkasten");
	let ignore: Vec<PathBuf> = vec![PathBuf::from(r"C:\Users\nickr\Documents\Vault\.obsidian")];
	let linker_options = LinkerOptions {
		link_share_tag: true,
		link_self: false
	};

	let mut vault = Vault::new(path, ignore, linker_options);
	vault.unlink_all_files();
	vault.link_all_files();
	let output = PathBuf::from("out");
	vault.export(&output);
	println!("Time elapsed: {:?}", start_time.elapsed());

}
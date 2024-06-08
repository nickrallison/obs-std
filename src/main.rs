
use crate::cli::{Cli, CLIError, run_cli};

use clap::Parser;

mod parse;
mod md_file;
mod vault;
mod linking;
mod cli;
mod stringtree;
mod test_suite;


fn main() {
	// let start_time = std::time::Instant::now();
	// let path = PathBuf::from(r"C:\Users\nickr\Documents\Vault\500-Zettelkasten");
	// let ignore: Vec<PathBuf> = vec![PathBuf::from(r"C:\Users\nickr\Documents\Vault\.obsidian")];
	// let linker_options = LinkerOptions {
	// 	link_share_tag: true,
	// 	link_self: false
	// };
	//
	// let mut vault = Vault::new(path, ignore, linker_options);
	// vault.unlink_all_files();
	// let path: PathBuf = PathBuf::from("Autocorrelation.md");
	// vault.link_all_files();
	// let output = PathBuf::from("out");
	// vault.export(&output);
	// println!("Time elapsed: {:?}", start_time.elapsed());


	// let vault_path = PathBuf::from(r"C:\Users\nickr\Documents\Vault\500-Zettelkasten");
	// let vault_path = PathBuf::from("test_vaults/full_vault");
	// let ignore: Vec<PathBuf> = vec![];
	// let linker_options = LinkerOptions {
	// 	link_share_tag: false,
	// 	link_self: false
	// };
	// let mut vault = crate::vault::Vault::new(vault_path, ignore, linker_options);
	// let alias_tree: String = vault.alias_tree_string();
	// println!("{}", alias_tree);
	//
	// vault.unlink_all_files();
	// vault.link_all_files();
	// let mut data: Vec<(PathBuf, MDFile)> = vault.data.clone().iter().map(|(path, md_file)| {
	// 	(path.clone(), md_file.clone())
	// }).collect();
	// data.sort_by(|a, b| a.0.cmp(&b.0));
	//
	// for (path, md_file) in data.iter() {
	// 	let result = md_file.to_string();
	// 	let expected = fs::read_to_string(md_file.path.clone().unwrap()).unwrap();
	//
	// 	if result != expected {
	// 		panic!("Failed on file: {}", md_file.path.clone().unwrap().display());
	// 	}
	// 	// assert_eq!(result, expected, "Failed on file: {}", md_file.path.clone().unwrap().display());
	// }
	// let output = PathBuf::from("out");
	// vault.export(&output);

	let cli = Cli::parse();
	let run_result: Result<(), CLIError> = run_cli(cli);
	match run_result {
		Ok(_) => {}
		Err(e) => {
			match e {
				CLIError::InvalidAction => {
					println!("Invalid Action");
				}
				CLIError::InvalidOptions => {
					println!("Invalid Options");
				}
				CLIError::InvalidTarget => {
					println!("Invalid Target");
				}
			}
		}
	}
}


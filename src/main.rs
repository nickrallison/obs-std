
use crate::cli::{CLI, CLIError, run_cli};

use clap::Parser;

mod parse;
mod md_file;
mod vault;
mod linking;
mod cli;
mod stringtree;
mod test_suite;


fn main() {

	let cli = CLI::parse();
	let run_result: Result<(), CLIError> = run_cli(cli);
	match run_result {
		Ok(()) => {}
		Err(e) => {
			match e {
				CLIError::InvalidTarget => {
					println!("Invalid Target");
				}
			}
		}
	}
}


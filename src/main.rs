
use crate::cli::{CLI, CLIError, run_cli};

use clap::Parser;
use crate::cli::Options::Time;

mod parse;
mod md_file;
mod vault;
mod linking;
mod cli;
mod stringtree;
mod test_suite;


fn main() {

	let cli = CLI::parse();
	let start = std::time::Instant::now();
	let options = cli.options.clone();
	let run_result: Result<(), CLIError> = run_cli(cli);
	match options {
		Time => {
			println!("Time: {}ms", start.elapsed().as_millis());
		}
		_ => {}
	}
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


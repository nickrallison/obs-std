use std::fmt::Display;
use std::path::PathBuf;
use std::{fs, result};
use clap::Parser;
use clap_derive::ValueEnum;
use difference::{Changeset, Difference};
use crate::linking::LinkerOptions;
use crate::vault::Vault;

// Actions
// #######
// None
// Linking
// Unlinking
// Alias Tree

// Options
// #######
// Preview - Defaults to False
//  - No file changes, just print what would happen
// Safe - Defaults to True
//  - Prompt before linking, change files
// Force - Defaults to False
//  - Dangerously link all files in place, no prompts (use with caution)

// Targets
// #######
// File
// All


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Action {
    None,
    Link,
    Unlink,
    AliasTree
}

impl Action {
    pub fn new(action: &str) -> Result<Self, String> {
        match action {
            "none" => Ok(Self::None),
            "link" => Ok(Self::Link),
            "unlink" => Ok(Self::Unlink),
            "alias_tree" => Ok(Self::AliasTree),
            _ => Err(format!("Invalid Action: {}", action))
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Link => write!(f, "Link"),
            Self::Unlink => write!(f, "Unlink"),
            Self::AliasTree => write!(f, "Alias Tree")
        }
    }
}

// String is the export path
/// Safety level when linking / unlinking files
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Options {
    Preview,
    Safe,
    Force
}

impl Options {
    pub fn new(options: &str) -> Result<Self, String> {
        match options {
            "preview" => Ok(Self::Preview),
            "safe" => Ok(Self::Safe),
            "force" => Ok(Self::Force),
            _ => Err(format!("Invalid Options: {}", options))
        }
    }
}

impl Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Preview => write!(f, "Preview"),
            Self::Safe => write!(f, "Safe"),
            Self::Force => write!(f, "Force")
        }
    }
}

#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[command(version, about, long_about = None)]
pub struct Cli {

    /// Action to perform
    #[arg(short, long)]
    pub action: Action,

    /// Safety level (Ignored for AliasTree)
    #[arg(short, long)]
    pub options: Options,

    /// Target file / folder
    #[arg(short, long)]
    pub target_path: String
}

impl Cli {
    pub fn new(action: Action, options: Options, target_path: String) -> Self {
        Self {
            action,
            options,
            target_path
        }
    }


    pub fn from_str(action: &str, options: &str, target_path: &str) -> Result<Self, String> {
        let action = Action::new(action);
        let options = Options::new(options);
        match (action, options) {
            (Ok(action), Ok(options)) => Ok(Self::new(action, options, target_path.to_string())),
            (Err(e1), Err(e2)) => Err(format!("{}\n{}", e1, e2)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e)
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CLIError {
    InvalidAction,
    InvalidOptions,
    InvalidTarget
}

pub fn run_cli(cli: Cli) -> Result<(), CLIError> {
    let action = cli.action;
    let options = cli.options;
    let target = cli.target_path;

    let path = PathBuf::from(target);
    if !path.exists() {
        return Err(CLIError::InvalidTarget);
    }

    let vault_path = path;
    let ignore: Vec<PathBuf> = vec![];
    let linker_options = LinkerOptions {
        link_share_tag: false,
        link_self: false
    };
    let mut vault = Vault::new(vault_path, ignore, linker_options);

    match action {
        Action::None => {}
        Action::Link => {
            vault.link_all_files_mut_ref();
        }
        Action::Unlink => {
            vault.unlink_all_files_mut_ref();
        }
        Action::AliasTree => {
            println!("{}", vault.alias_tree_string());
            return Ok(());
        }
    }

    match options {
        Options::Preview => {
            for (_, mdfile) in vault.data.iter() {
                println!("{}\n##########################", mdfile);
            }
        }
        Options::Safe => {
            for (_, mdfile) in vault.data.iter() {
                let path = mdfile.path.clone().unwrap();
                let expected = fs::read_to_string(&path).unwrap();
                let actual = mdfile.to_string();
                if expected != actual {
                    // Print diff
                    print_diff(&expected, &actual);

                    // prompt user whether or not to accept changes
                    println!("\n##### EOF #####\nDo you want to accept these changes to file: {}? yes, no, or cancel [y/n/c]: ", mdfile.path.as_ref().expect("MDFile does not have path set").display());
                    let mut input = String::new();
                    while input.trim() != "y" && input.trim() != "n" && input.trim() != "c" {
                        std::io::stdin().read_line(&mut input).unwrap();
                        if input.trim() == "y" {
                            fs::write(&path, actual).unwrap();
                            break;
                        }
                        else if input.trim() == "c" {
                            return Ok(());
                        }
                        else if input.trim() == "n" {
                            break;
                        } else {
                            println!("Invalid input: [{}]. Please enter y, n, or c: ", input.trim());
                            input.clear();
                        }
                    }
                }
            }
        }
        Options::Force => {
            println!("\nYou are about to modify these files under this path: {}. Are you sure you want to do this? yes, or no [y/n]: ", vault.get_path().display());
            let mut input = String::new();
            while input.trim() != "y" && input.trim() != "n" {
                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim() == "y" {
                    for (_, mdfile) in vault.data.iter() {
                        let path = mdfile.path.clone().unwrap();
                        let actual = mdfile.to_string();
                        fs::write(&path, actual).unwrap();
                    }
                }
                else if input.trim() == "n" {
                    return Ok(());
                } else {
                    println!("Invalid input: [{}]. Please enter y, or n: ", input.trim());
                    input.clear();
                }
            }
        }
    }

    Ok(())
}

fn print_diff(str1: &String, str2: &String) {
    let changeset = Changeset::new(str1, str2, "");
    for diff in changeset.diffs {
        match diff {
            Difference::Same(part) => print!("{}", part),
            Difference::Add(part) => print!("\x1b[92m{}\x1b[0m", part),
            Difference::Rem(part) => print!("\x1b[91m{}\x1b[0m", part),
        }
    }
}

#[cfg(test)]
mod cli_tests {
    use crate::cli::{Cli, CLIError, run_cli};

    #[test]
    fn test_cli_folder_does_exist() {
        let cli_res = Cli::from_str("alias_tree", "preview", "test_vaults/full_vault");
        let cli = match cli_res {
            Ok(cli) => cli,
            Err(e) => {
                panic!("Test failed to create cli: {}", e);
            }
        };
        let run_result: Result<(), CLIError> = run_cli(cli);
        match run_result {
            Ok(_) => {}
            Err(e) => {
                match e {
                    CLIError::InvalidAction => {
                        panic!("Test got invalid action error when it should not have.");
                    }
                    CLIError::InvalidOptions => {
                        panic!("Test got invalid options error when it should not have.");
                    }
                    CLIError::InvalidTarget => {
                        panic!("Test got invalid target error when it should not have.");
                    }
                }
            }
        }
    }
    #[test]
    fn test_cli_folder_does_not_exist() {
        let cli_res = Cli::from_str("alias_tree", "preview", "test_vaults/bad_vault");
        let cli = match cli_res {
            Ok(cli) => cli,
            Err(e) => {
                panic!("Test failed to create cli: {}", e);
            }
        };
        let run_result: Result<(), CLIError> = run_cli(cli);
        match run_result {
            Ok(_) => {
                panic!("CLI run should not have succeeded.")
            }
            Err(e) => {
                match e {
                    CLIError::InvalidAction => {
                        panic!("Test got invalid action error when it should not have.");
                    }
                    CLIError::InvalidOptions => {
                        panic!("Test got invalid options error when it should not have.");
                    }
                    CLIError::InvalidTarget => {}
                }
            }
        }
    }
}
use std::fmt::Display;
use std::path::PathBuf;
use std::fs;
use clap::Parser;
use clap_derive::ValueEnum;
use difference::{Changeset, Difference};
use crate::linking::LinkerOptions;
use crate::vault::Vault;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, ValueEnum)]
pub enum Action {
    #[default]
    None,
    Link,
    Unlink,
    AliasTree
}

impl Action {
    #[allow(dead_code)]
    pub fn new(action: &str) -> Result<Self, String> {
        match action {
            "none" => Ok(Self::None),
            "link" => Ok(Self::Link),
            "unlink" => Ok(Self::Unlink),
            "alias_tree" => Ok(Self::AliasTree),
            _ => Err(format!("Invalid Action: {action}"))
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, ValueEnum)]
pub enum RunOption {
    #[default]
    Preview,
    Safe,
    Force,

    Time
}

impl RunOption {
    #[allow(dead_code)]
    pub fn new(options: &str) -> Result<Self, String> {
        match options {
            "preview" => Ok(Self::Preview),
            "safe" => Ok(Self::Safe),
            "force" => Ok(Self::Force),
            "time" => Ok(Self::Time),
            _ => Err(format!("Invalid Options: {options}"))
        }
    }
}

impl Display for RunOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Preview => write!(f, "Preview"),
            Self::Safe => write!(f, "Safe"),
            Self::Force => write!(f, "Force"),
            Self::Time => write!(f, "Time")
        }
    }
}

#[derive(Parser, Debug, Clone, PartialEq, Eq, Default)]
#[command(version, about, long_about = None)]
pub struct CLI {

    /// Action to perform
    #[arg(short, long)]
    pub action: Action,

    /// Safety level (Ignored for `AliasTree`)
    #[arg(short, long)]
    pub options: RunOption,

    /// Target file / folder
    #[arg(short, long)]
    pub target_path: String,

    /// db path
    #[arg(short, long)]
    pub db_path: Option<String>,

    /// ignored paths
    #[arg(short, long)]
    pub ignore: Vec<String>
}

impl CLI {
    #[allow(dead_code)]
    pub fn new(action: Action, options: RunOption, target_path: String, db_path: Option<String>, ignore: Vec<String>) -> Self {
        Self {
            action,
            options,
            target_path,
            db_path,
            ignore
        }
    }

    #[allow(dead_code)]
    pub fn from_str(action: &str, options: &str, target_path: &str) -> Result<Self, String> {
        let action = Action::new(action);
        let options = RunOption::new(options);
        match (action, options) {
            (Ok(action), Ok(options)) => Ok(Self::new(action, options, target_path.to_string(), None, vec![])),
            (Err(e1), Err(e2)) => Err(format!("{e1}\n{e2}")),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e)
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CLIError {
    InvalidTarget,
    InvalidDB(String),
    DBWriteError(String)
}

#[allow(dead_code)]
pub fn run_cli(cli: CLI) -> Result<(), CLIError> {
    let action = cli.action;
    let options = cli.options;
    let target = cli.target_path;
    let db_path = cli.db_path;
    let ignore = cli.ignore;

    let db_path: Option<PathBuf> = match db_path {
        Some(path) => {
            let path = PathBuf::from(path);

            // if path is a file, Some(path)
            if path.is_file() {
                Some(path)
            }
            // if path is a directory, return Err(
            else if path.is_dir() {
                return Err(CLIError::InvalidDB(format!("DB path must not be an existing Directory, {} is directory.", path.display())));
            }

            // if path does not exist but parent directory does, return Some(path)
            else if path.parent().unwrap().is_dir() {
                Some(path)
            }

            // if path does not exist but parent is file, return Err
            else if path.parent().unwrap().is_file() {
                let parent = path.parent().unwrap();
                return Err(CLIError::InvalidDB(format!("DB path must be a valid path, {} is a file.", parent.display())));
            }

            // if path does not exist and parent directory does not exist, return Err
            else {
                let parent = path.parent().unwrap();
                return Err(CLIError::InvalidDB(format!("DB path must be a valid path, parent directory does not exist: {}", parent.display())));
            }
        }
        None => None
    };

    let path = PathBuf::from(target);
    if !path.exists() {
        return Err(CLIError::InvalidTarget);
    }

    let vault_path = path;
    let linker_options = LinkerOptions {
        link_share_tag: false,
        link_self: false
    };

    let ignore: Vec<PathBuf> = ignore.iter().map(|x| PathBuf::from(x)).collect();

    let mut vault = Vault::new(vault_path, ignore, linker_options, db_path.clone());

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
        RunOption::Preview => {
            for (_, mdfile) in &vault.data {
                println!("{mdfile}\n##########################");
            }
        }
        RunOption::Safe => {
            for (_, mdfile) in &vault.data {
                let path = mdfile.path.clone();
                let expected = fs::read_to_string(&path).unwrap();
                let actual = mdfile.to_string();
                if expected != actual {
                    // Print diff
                    print_diff(&expected, &actual);

                    // prompt user whether or not to accept changes
                    println!("\n##### EOF #####\nDo you want to accept these changes to file: {}? yes, no, or cancel [y/n/c]: ", mdfile.path.display());
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
        RunOption::Force => {
            // println!("\nYou are about to modify these files under this path: {}. Are you sure you want to do this? yes, or no [y/n]: ", vault.get_path().display());
            // let mut input = String::new();
            // while input.trim() != "y" && input.trim() != "n" {
            //     std::io::stdin().read_line(&mut input).unwrap();
            //     if input.trim() == "y" {
            //         for (_, mdfile) in &vault.data {
            //             fs::write(&mdfile.path, mdfile.to_string()).unwrap();
            //         }
            //     }
            //     else if input.trim() == "n" {
            //         return Ok(());
            //     } else {
            //         println!("Invalid input: [{}]. Please enter y, or n: ", input.trim());
            //         input.clear();
            //     }
            // }
            for (path, mdfile) in &vault.data {
                fs::write(&mdfile.path, mdfile.to_string()).unwrap();
                println!("Wrote to file: {}", path.display());
            }
        }
        RunOption::Time => {}
    }

    match db_path {
        Some(path) => {
            let db_path = path;
            let db_path = PathBuf::from(db_path);
            let res = vault.to_json(&db_path);
            match res {
                Ok(_) => {}
                Err(e) => {
                    return Err(CLIError::DBWriteError(format!("{e}")));
                }
            }
        }
        None => {}
    }

    Ok(())
}

fn print_diff(str1: &String, str2: &String) {
    let changeset = Changeset::new(str1, str2, "");
    for diff in changeset.diffs {
        match diff {
            Difference::Same(part) => print!("{part}"),
            Difference::Add(part) => print!("\x1b[92m{part}\x1b[0m"),
            Difference::Rem(part) => print!("\x1b[91m{part}\x1b[0m"),
        }
    }
}

#[cfg(test)]
mod cli_tests {
    use crate::cli::{CLI, CLIError, run_cli};

    #[test]
    fn test_cli_folder_does_exist() {
        let cli_res = CLI::from_str("alias_tree", "preview", "test_vaults/reference_clean");
        let cli = match cli_res {
            Ok(cli) => cli,
            Err(e) => {
                panic!("Test failed to create cli: {e}");
            }
        };
        let run_result: Result<(), CLIError> = run_cli(cli);
        match run_result {
            Ok(()) => {}
            Err(e) => {
                match e {
                    CLIError::InvalidTarget => {
                        panic!("Test got invalid target error when it should not have.");
                    }
                    CLIError::InvalidDB(_) => {
                        panic!("Test got invalid db error when it should not have.");
                    }
                    CLIError::DBWriteError(_) => {
                        panic!("Test got db write error when it should not have.");
                    }
                }
            }
        }
    }
    #[test]
    fn test_cli_folder_does_not_exist() {
        let cli_res = CLI::from_str("alias_tree", "preview", "test_vaults/reference_bad");
        let cli = match cli_res {
            Ok(cli) => cli,
            Err(e) => {
                panic!("Test failed to create cli: {e}");
            }
        };
        let run_result: Result<(), CLIError> = run_cli(cli);
        match run_result {
            Ok(()) => {
                panic!("CLI run should not have succeeded.")
            }
            Err(e) => {
                match e {
                    CLIError::InvalidTarget => {}
                    CLIError::InvalidDB(_) => {
                        panic!("Test got invalid db error when it should not have.");
                    }
                    CLIError::DBWriteError(_) => {
                        panic!("Test got db write error when it should not have.");
                    }
                }
            }
        }
    }
}
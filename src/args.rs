use std::path::PathBuf;
use clap::Parser;
use clap_derive::{Subcommand, ValueEnum};

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

// String is the export path
/// Safety level when linking / unlinking files
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Options {
    Preview,
    Safe,
    Force
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {

    /// Action to perform
    #[arg(short, long)]
    pub action: Action,

    /// Safety level
    #[arg(short, long)]
    pub options: Options,

    /// Target file / folder
    #[arg(short, long)]
    pub target_path: String
}
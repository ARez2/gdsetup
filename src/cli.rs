use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, builder};


#[cfg_attr(name, String)]
#[derive(Parser, Debug)]
#[command(about = "Sets up a Godot C++ project.", long_about = None)]
#[command(author, version, about, long_about = None)]
pub struct GDSetup {
    /// Optional: Shorthand version for 'gdsetup init [NAME]'
    pub name: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,


    #[arg(
        long,
        short,
        default_value_t = String::from("trace"),
        value_parser = clap::builder::PossibleValuesParser::new(["error", "warn", "info", "debug", "trace"]),
    )]
    pub log_level: String,
}

#[derive(Args, Debug)]
pub struct RenameArgs {
    pub name: String,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    pub name: String,
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Optional: A shorthand version for [--path | -p]
    pub path: Option<PathBuf>,

    /// Path to an existing Godot project folder
    #[arg(long = "path", short = 'p', value_name = "SPECIFIC PATH")]
    pub path2: Option<PathBuf>,

    /// Whether to use the 'scons' command to instantly build the project once it has been initialized.
    #[arg(long, short, default_value_t = true)]
    pub build: bool,
}


#[derive(Subcommand, Debug)]
pub enum Commands {
    /// IF project.godot: Move everything into godot/    ELSE: Creates the folders and example dummy files
    Init(InitArgs),
    /// Renames all occurences of the extension name (or the files)
    Rename(RenameArgs),
    /// Creates another GDExtension module
    Add(AddArgs),
}
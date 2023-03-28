use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};


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
    /// Name of the GDExtension module you want to rename
    pub old_modulename: String,
    /// New name
    pub new_modulename: String,

    /// Path to an existing Godot GDExtension project folder
    #[arg(long = "path", short = 'p', value_name = "PATH")]
    pub path: Option<PathBuf>,

    /// Whether to NOT use the 'scons' command to instantly build the project once it has been initialized.
    #[arg(long = "no-build", short = 'b', default_value_t = false)]
    pub nobuild: bool,
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

    /// Whether to NOT use the 'scons' command to instantly build the project once it has been initialized.
    #[arg(long = "no-build", short = 'b', default_value_t = false)]
    pub nobuild: bool,
}


#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Creates or initializes a folder to be used as a Godot GDExtension project. If a folder b
    /// 
    /// Examples:
    /// 
    ///     'gdsetup mynewproject'             - Creates a new folder named 'mynewproject' and all relevant subfolders
    /// 
    ///     'gdsetup init mynewproject'        - Longer form of above command
    /// 
    ///     'gdsetup init -p mynewproject'     - Longest form of above command
    /// 
    ///     'gdsetup init mynewproject -b'     - Creates a new folder named 'mynewproject' (+subfolders) but does NOT instantly build it
    Init(InitArgs),
    /// Renames all occurences of the extension name (or the files)
    /// To do that, gdsetup tries to find certain files and functions it has generated upon running 'gdsetup init'.
    /// If you have changed function names or file names, this might not work as intended and manual work is needed.
    /// 
    /// Examples:
    /// 
    ///     'gdsetup rename oldname newname -p path/to/project'             - Performs the renaming process inside the path/to/project folder
    /// 
    ///     'gdsetup rename oldname newname -p path/to/project'             - Performs the renaming process inside the path/to/project folder WITHOUT building
    Rename(RenameArgs),
    /// Creates another GDExtension module
    Add(AddArgs),
}
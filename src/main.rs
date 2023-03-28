use std::{str::FromStr};

use anyhow::{Result, Error};
use clap::Parser;
use log::debug;

use gdsetup::{init, Commands, GDSetup, InitArgs, add_extension, rename};

const GODOT_DIR: &str = "godot";
const SRC_DIR: &str = "src";
const CLASS_NAME: &str = "example";
const DEFAULT_PROJECTNAME: String = String::new();


// Create folders:
//      godot
//      godot-cpp (Submodule)
//      src
// Create/ Manage:
//      SConstruct
//      CMakeLists.txt
//      godot/*.extension
//      godot/.godot/extension_list.cfg
//      src/register_types.[cpp/h]
//      src/extension.[cpp/h]
//      (src/.vscode)

// Commands needed:
//      gdsetup [init path/to/folder/]         IF project.godot: Move everything into godot/    ELSE: Creates the folders and example dummy files
//      gdsetup rename NewExtensionName        Renames all occurences of the extension name (or the files)
//      gdsetup add NewExtensionName           Creates another Extension



fn main() -> Result<(), Error> {
    let args = GDSetup::parse();
    let level = log::LevelFilter::from_str(args.log_level.as_str())?;
    std::env::set_var("RUST_LOG", format!("={}", level));
    env_logger::init_from_env(env_logger::Env::new());

    debug!("{args:?}");
    if let Some(command) = args.command {
        match command {
            Commands::Init(pathargs) => init(pathargs, args.name, CLASS_NAME, GODOT_DIR, SRC_DIR),
            Commands::Rename(nameargs) => rename(nameargs),
            Commands::Add(nameargs) => add_extension(nameargs),
        }
    } else {
        init(InitArgs { path: None, path2: None, nobuild: false}, args.name, CLASS_NAME, GODOT_DIR, SRC_DIR)
    }

}

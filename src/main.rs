use std::{path::{PathBuf}, str::FromStr};

use anyhow::{Result, Error, Context, Ok};
use clap::{Args, Parser, Subcommand, builder};
use log::{debug, info, warn};

use gdsetup::codegen;

const GODOT_DIR: &'static str = "godot";
const SRC_DIR: &'static str = "src";
const CLASS_NAME: &'static str = "example";


#[cfg_attr(name, String)]
#[derive(Parser, Debug)]
#[command(about = "Sets up a Godot C++ project.", long_about = None)]
#[command(author, version, about, long_about = None)]
struct GDSetup {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(
        long,
        short,
        default_value_t = String::from("info"),
        value_parser = clap::builder::PossibleValuesParser::new(["error", "warn", "info", "debug", "trace"]),
    )]
    log_level: String,
}

#[derive(Args, Debug)]
struct NameArgs {
    name: String,
}

#[derive(Args, Debug)]
struct PathArgs {
    path: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// IF project.godot: Move everything into godot/    ELSE: Creates the folders and example dummy files
    Init(PathArgs),
    /// Creates another Extension
    Add(NameArgs),
    /// Renames all occurences of the extension name (or the files)
    Rename(NameArgs),
}




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

    if let std::result::Result::Ok(_) = std::fs::remove_dir_all(format!(".\\test\\testproject\\{}", GODOT_DIR).as_str()) {

    };
    if let std::result::Result::Ok(_) = std::fs::remove_dir_all(format!(".\\test\\testproject\\{}", SRC_DIR).as_str()) {

    };
    //return Ok(());
    
    std::fs::copy(".\\test\\project.godot", ".\\test\\testproject\\project.godot")?;
    std::fs::copy(".\\test\\test.txt", ".\\test\\testproject\\test.txt")?;
    //return Ok(());

    debug!("{args:?}");
    if let Some(command) = args.command {
        match command {
            Commands::Init(pathargs) => init(pathargs),
            Commands::Add(nameargs) => add_extension(nameargs),
            Commands::Rename(nameargs) => rename(nameargs),
        }
    } else {
        init(PathArgs { path: None })
    }

}

// return Err(Error::msg(format!("Directory `{}` is not empty. If you want to continue add the -f (--force) option", pathstr)));
fn init(pathargs: PathArgs) -> Result<(), Error> {
    let path = match pathargs.path {
        Some(path) => path,
        None => PathBuf::from(std::env::current_dir().unwrap_or_default())
    };
    let pathstr = path.to_str().unwrap();
    let read = std::fs::read_dir(path.clone())?;
    let read = read.into_iter().flatten().collect::<Vec<std::fs::DirEntry>>();

    let godot_folder = path.clone().join(GODOT_DIR);
    let godot_folder_str = godot_folder.clone().display().to_string();
    std::fs::create_dir(godot_folder.clone()).with_context(|| format!("Failed to create directory '{}'", godot_folder_str))?;

    let project_exists = read.iter().find(|f| f.file_name() == "project.godot").is_some();
    if project_exists {
        info!("Moving everything inside '{}' into '{}'", pathstr, godot_folder_str);
        for file in read.iter() {
            let orig = path.join(file.file_name());
            let orig_disp = orig.display().to_string();
            let dest = godot_folder.clone().join(file.file_name());
            let dest_disp = dest.display().to_string();
            info!("Copying `{}` into `{}` ...", orig_disp, dest_disp);
            std::fs::copy(orig, dest).with_context(|| format!("Tried copying {} into {}", orig_disp, dest_disp))?;
        };
        // Godot project folder has been created
        for file in read.iter() {
            let filename = path.clone().join(file.file_name());
            info!("Removing '{:?}' from '{}' ...", file.file_name().to_str().unwrap(), pathstr);
            std::fs::remove_file(filename).with_context(|| format!("Failed to remove '{:?}' from {}", file.file_name().to_str().unwrap(), pathstr))?;
        };
    };
    
    let src_folder = path.clone().join(SRC_DIR);
    let src_folder_str = src_folder.clone().display().to_string();
    std::fs::create_dir(src_folder.clone()).with_context(|| format!("Failed to create directory '{}'", src_folder_str))?;

    std::fs::write(src_folder.clone().join("register_types.cpp"), codegen::generate_register_cpp(CLASS_NAME))?;

    std::fs::write(src_folder.clone().join("register_types.h"), codegen::generate_register_h(CLASS_NAME))?;
    
    std::fs::write(src_folder.clone().join(format!("{}.cpp", CLASS_NAME)), codegen::generate_class_cpp(CLASS_NAME))?;
    
    std::fs::write(src_folder.clone().join(format!("{}.h", CLASS_NAME)), codegen::generate_class_h(CLASS_NAME))?;
    Ok(())
}


fn add_extension(nameargs: NameArgs) -> Result<(), Error> {
    Ok(())
}


fn rename(nameargs: NameArgs) -> Result<(), Error> {
    Ok(())
}
use std::{path::{PathBuf, self}, str::FromStr};

use anyhow::{Result, Error, Context, Ok};
use clap::{Args, Parser, Subcommand, builder};
use log::{debug, info, warn};
use std::process::Command;

use gdsetup::codegen;

const GODOT_DIR: &'static str = "godot";
const SRC_DIR: &'static str = "src";
const CLASS_NAME: &'static str = "example";
const DEFAULT_PROJECTNAME: String = String::new();


#[cfg_attr(name, String)]
#[derive(Parser, Debug)]
#[command(about = "Sets up a Godot C++ project.", long_about = None)]
#[command(author, version, about, long_about = None)]
struct GDSetup {
    /// Optional: Shorthand version for 'gdsetup init [NAME]'
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,


    #[arg(
        long,
        short,
        default_value_t = String::from("trace"),
        value_parser = clap::builder::PossibleValuesParser::new(["error", "warn", "info", "debug", "trace"]),
    )]
    log_level: String,
}

#[derive(Args, Debug)]
struct RenameArgs {
    name: String,
}

#[derive(Args, Debug)]
struct AddArgs {
    name: String,
}

#[derive(Args, Debug)]
struct InitArgs {
    /// Optional: A shorthand version for [--path | -p]
    path: Option<PathBuf>,

    /// Path to an existing Godot project folder
    #[arg(long = "path", short = 'p', value_name = "SPECIFIC PATH")]
    path2: Option<PathBuf>,
}


#[derive(Subcommand, Debug)]
enum Commands {
    /// IF project.godot: Move everything into godot/    ELSE: Creates the folders and example dummy files
    Init(InitArgs),
    /// Renames all occurences of the extension name (or the files)
    Rename(RenameArgs),
    /// Creates another GDExtension module
    Add(AddArgs),
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

    debug!("{args:?}");
    if let Some(command) = args.command {
        match command {
            Commands::Init(pathargs) => init(pathargs, args.name),
            Commands::Rename(nameargs) => rename(nameargs),
            Commands::Add(nameargs) => add_extension(nameargs),
        }
    } else {
        init(InitArgs { path: None, path2: None}, args.name)
    }

}

// return Err(Error::msg(format!("Directory `{}` is not empty. If you want to continue add the -f (--force) option", pathstr)));
fn init(pathargs: InitArgs, projectname: Option<String>) -> Result<(), Error> {
    // Decide whether to use the shorthand path version if provided
    let p = {
        if pathargs.path.is_some() {
            pathargs.path
        } else if pathargs.path2.is_some() {
            pathargs.path2
        } else {
            None
        }
    };

    // One of them must be given
    // TODO: Maybe allow this but with a confirmation as it could change the current folder
    if p.is_none() && projectname.is_none() {
        return Err(Error::msg("Missing either a --path (-p) to an exisiting folder or a name for a new one."))
    };

    debug!("Testing whether the 'git' command exists");
    let git = Command::new("git").output().with_context(|| "Tried to find git").unwrap();

    let current_dir = std::env::current_dir().unwrap_or_default();

    // Create a new project folder
    let path = if let Some(name) = projectname {
        let dir = current_dir.join(name);
        let _d = dir.clone();
        let dirname = _d.display();
        info!("Creating new project folder '{}'", dirname);
        std::fs::create_dir(dir.clone()).with_context(|| format!("Tried to create a new folder '{}'", dirname))?;
        dir
    } else {
        match p {
            Some(path) => path,
            None => current_dir,
        }
    };
    let pathstr = path.to_str().unwrap();
    let read = std::fs::read_dir(path.clone())?;
    let read = read.into_iter().flatten().collect::<Vec<std::fs::DirEntry>>();

    // Create the 'godot' folder
    let godot_folder = path.clone().join(GODOT_DIR);
    let godot_folder_str = godot_folder.clone().display().to_string();
    std::fs::create_dir(godot_folder.clone()).with_context(|| format!("Failed to create directory '{}'", godot_folder_str))?;

    let project_exists = read.iter().find(|f| f.file_name() == "project.godot").is_some();
    // If there's a project.godot: Move everything to 'godot/'
    if project_exists {
        info!("Found project.godot. Moving everything inside '{}' into '{}'", pathstr, godot_folder_str);
        for file in read.iter() {
            let orig = path.join(file.file_name());
            let orig_disp = orig.display().to_string();
            let dest = godot_folder.clone().join(file.file_name());
            let dest_disp = dest.display().to_string();
            info!("Copying `{}` into `{}` ...", orig_disp, dest_disp);
            std::fs::copy(orig, dest).with_context(|| format!("Tried copying {} into {}", orig_disp, dest_disp))?;
        };
        // Godot project folder has been created, remove old files
        for file in read.iter() {
            let filename = path.clone().join(file.file_name());
            info!("Removing {:?} from '{}' ...", file.file_name().to_str().unwrap(), pathstr);
            std::fs::remove_file(filename).with_context(|| format!("Failed to remove '{:?}' from {}", file.file_name().to_str().unwrap(), pathstr))?;
        };
    };

    // Create the godot-relevant files for the extension
    std::fs::write(godot_folder.clone().join(format!("{}.gdextension", CLASS_NAME)), codegen::generate_gdextension(CLASS_NAME))
        .with_context(||  format!("Tried creating the '{}.gdextension' file.", CLASS_NAME))?;
    std::fs::create_dir(godot_folder.clone().join(".godot"))
        .with_context(|| "Tried creating a .godot folder")?;
    std::fs::write(godot_folder.clone().join(".godot").join("extension_list.cfg"), codegen::generate_gdextension_list(CLASS_NAME))
        .with_context(|| "Tried creating '.godot/extension_list.cfg'")?;
    
    // Create the 'src/' folder
    let src_folder = path.clone().join(SRC_DIR);
    let src_folder_str = src_folder.clone().display().to_string();
    std::fs::create_dir(src_folder.clone()).with_context(|| format!("Failed to create directory '{}'", src_folder_str))?;

    // Create compilation files
    std::fs::write(path.clone().join("SConstruct"), codegen::generate_sconstruct(CLASS_NAME))?;
    std::fs::write(path.clone().join("CMakeLists.txt"), codegen::generate_cmakelists(CLASS_NAME))?;

    // Create the registration files
    std::fs::write(src_folder.clone().join("register_types.cpp"), codegen::generate_register_cpp(CLASS_NAME))?;
    std::fs::write(src_folder.clone().join("register_types.h"), codegen::generate_register_h(CLASS_NAME))?;
    // Create the class files
    std::fs::write(src_folder.clone().join(format!("{}.cpp", CLASS_NAME)), codegen::generate_class_cpp(CLASS_NAME))?;
    std::fs::write(src_folder.clone().join(format!("{}.h", CLASS_NAME)), codegen::generate_class_h(CLASS_NAME))?;


    let basecmd = {
        if cfg!(target_family = "windows") {
            ("cmd", "/C")
        } else if cfg!(target_family = "unix") {
            ("sh", "-c")
        } else { // WASM - also unix??
            ("sh", "-c")
        }
    };

    info!("Running 'git init'");
    let output = Command::new(basecmd.0).arg(basecmd.1)
        .current_dir(pathstr)
        .arg("git").arg("init").output().with_context(|| "Tried to 'git init'")?;
    _ = print_output(output);
    
    info!("Running 'git submodule add https://github.com/godotengine/godot-cpp.git'");
    let output = Command::new(basecmd.0).arg(basecmd.1)
        .current_dir(pathstr)
        .arg("git").args(["submodule", "add", "https://github.com/godotengine/godot-cpp.git"]).output().with_context(|| "Tried to find git")?;
    _ = print_output(output);

    info!("Running 'scons'");
    let output = Command::new(basecmd.0).arg(basecmd.1)
        .current_dir(pathstr)
        .arg("scons").output();
    if let std::result::Result::Ok(output) = output {
        //_ = print_output(output);
    };

    Ok(())
}


fn print_output(output: std::process::Output) -> Result<(), Error> {
    if !output.stdout.is_empty() {
        debug!("{}", String::from_utf8(output.stdout)?);
    };
    if !output.stderr.is_empty() {
        debug!("{}", String::from_utf8(output.stderr)?);
    };
    Ok(())
}


fn rename(nameargs: RenameArgs) -> Result<(), Error> {
    Ok(())
}


fn add_extension(nameargs: AddArgs) -> Result<(), Error> {
    Ok(())
}
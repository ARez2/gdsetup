use std::process::Command;

use crate::{cli::*, codegen, print_output};
use anyhow::{Result, Error, Context, Ok};
use log::{info, debug, warn};

pub fn init(pathargs: InitArgs, projectname: Option<String>, classname: &str, godot_dir: &str, src_dir: &str) -> Result<(), Error> {
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

    let git_exists = Command::new("git").output().with_context(|| "Tried to find git").is_ok();
    debug!("Testing whether the 'git' command exists: {}", git_exists);

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
    let godot_folder = path.clone().join(godot_dir);
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
    std::fs::write(godot_folder.clone().join(format!("{}.gdextension", classname)), codegen::generate_gdextension(classname))
        .with_context(||  format!("Tried creating the '{}.gdextension' file.", classname))?;
    std::fs::create_dir(godot_folder.clone().join(".godot"))
        .with_context(|| "Tried creating a .godot folder")?;
    std::fs::write(godot_folder.clone().join(".godot").join("extension_list.cfg"), codegen::generate_gdextension_list(classname))
        .with_context(|| "Tried creating '.godot/extension_list.cfg'")?;
    
    // Create the 'src/' folder
    let src_folder = path.clone().join(src_dir);
    let src_folder_str = src_folder.clone().display().to_string();
    std::fs::create_dir(src_folder.clone()).with_context(|| format!("Failed to create directory '{}'", src_folder_str))?;

    // Create compilation files
    std::fs::write(path.clone().join("SConstruct"), codegen::generate_sconstruct(classname))?;
    std::fs::write(path.clone().join("CMakeLists.txt"), codegen::generate_cmakelists(classname))?;

    // Create the registration files
    std::fs::write(src_folder.clone().join("register_types.cpp"), codegen::generate_register_cpp(classname))?;
    std::fs::write(src_folder.clone().join("register_types.h"), codegen::generate_register_h(classname))?;
    // Create the class files
    std::fs::write(src_folder.clone().join(format!("{}.cpp", classname)), codegen::generate_class_cpp(classname))?;
    std::fs::write(src_folder.clone().join(format!("{}.h", classname)), codegen::generate_class_h(classname))?;


    let basecmd = {
        if cfg!(target_family = "windows") {
            ("cmd", "/C")
        } else if cfg!(target_family = "unix") {
            ("sh", "-c")
        } else { // WASM - also unix??
            ("sh", "-c")
        }
    };

    // Git does not exist, we can't get the godot-cpp submodule and therefore can't build it
    if git_exists {
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
        
        if pathargs.build {
            info!("Running 'scons'");
            let output = Command::new(basecmd.0).arg(basecmd.1)
                .current_dir(pathstr)
                .arg("scons").output();
            if let std::result::Result::Ok(output) = output {
                //_ = print_output(output);
            };
        };
    } else {
        warn!("Did not find the 'git' command. Make sure git is installed to get the 'godot-cpp' submodule and build-support via scons")
    };

    Ok(())
}
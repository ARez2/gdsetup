use std::{path::PathBuf, fmt::format, process::Command};

use anyhow::{Result, Error, Context};
use log::{info, warn};

use crate::{RenameArgs, codegen, get_basecommand, print_output};


fn rename_file_contents(filepath: PathBuf, replacements: Vec<(String, String)>) {
    let filename = filepath.file_name().unwrap().to_str().unwrap().to_string();

    match std::fs::read_to_string(filepath.clone()) {
        Ok(val) => {
            let mut contents = val;
            for (old_value, new_value) in replacements {
                contents = contents.replace(&old_value, &new_value);
            };
            let write = std::fs::write(filepath, contents);
            match write {
                Ok(_) => {
                    info!("Successfully renamed contents of {}", filename);
                },
                Err(err) => {
                    warn!("An error occured while renaming contents of {}: {}", filename, err);
                },
            }
        },
        Err(err) => {
            warn!("An error occured while reading {}: {}", filename, err);
        }
    }
}


pub fn rename(renameargs: RenameArgs) -> Result<(), Error> {
    let path = renameargs.path.with_context(|| "rename: Tried getting the path argument").unwrap();

    let pathstr = path.to_str().unwrap();
    let read = std::fs::read_dir(path.clone())
        .with_context(|| format!("Tried reading {}", pathstr))?
        .into_iter().flatten()
        .collect::<Vec<std::fs::DirEntry>>();

    for file in read {
        match file.file_name().to_str().unwrap() {
            "godot" => {
                continue;
                let extension_list_path = file.path().join(".godot/extension_list.cfg");
                rename_file_contents(
                    extension_list_path,
                    vec![
                        (format!("{}.gdextension", renameargs.old_modulename), format!("{}.gdextension", renameargs.new_modulename)),
                    ]
                );
                
                let module_path = file.path().join(format!("{}.gdextension", renameargs.old_modulename));
                rename_file_contents(
                    module_path.clone(),
                    vec![
                        (format!("{}_library_init", renameargs.old_modulename), format!("{}_library_init", renameargs.new_modulename)),
                        (format!("libgd{}.", renameargs.old_modulename), format!("libgd{}.", renameargs.new_modulename))
                    ]
                );
                let new_module_path = file.path().join(format!("{}.gdextension", renameargs.new_modulename));
                std::fs::rename(
                    module_path.clone(),
                    new_module_path.clone(),
                ).with_context(|| format!("Tried renaming {} to {}", module_path.display(), new_module_path.display())).unwrap_or_else(|_| ());
            },
            "CMakeLists.txt" => {
                let cmake_path = file.path();
                rename_file_contents(
                    cmake_path.clone(),
                    vec![
                        (format!("project({})", renameargs.old_modulename), format!("project({})", renameargs.new_modulename)),
                        (format!("PROPERTY OUTPUT_NAME \"{}\"", renameargs.old_modulename), format!("PROPERTY OUTPUT_NAME \"{}\"", renameargs.new_modulename))
                    ]
                );
            },
            "SConstruct" => {
                let sconstruct_path = file.path();
                rename_file_contents(
                    sconstruct_path,
                    vec![
                        (format!("libgd{}", renameargs.old_modulename), format!("libgd{}", renameargs.new_modulename))
                    ]
                );
            },
            "src" => {
                let old_classname = codegen::get_classname_uppercase(&renameargs.old_modulename);
                let new_classname = codegen::get_classname_uppercase(&renameargs.new_modulename);
                let old_uppercase = renameargs.old_modulename.to_uppercase();
                let new_uppercase = renameargs.new_modulename.to_uppercase();

                
                // Rename class in the respective CPP file
                let old_class_cpp_path = file.path().join(format!("{}.cpp", renameargs.old_modulename));
                let class_cpp_path = file.path().join(format!("{}.cpp", renameargs.new_modulename));
                rename_file_contents(
                    old_class_cpp_path.clone(),
                    vec![
                        (format!("#include \"{}.h\"", renameargs.old_modulename), format!("#include \"{}.h\"", renameargs.new_modulename)),
                        (format!("{}::{}()", old_classname, old_classname), format!("{}::{}()", new_classname, new_classname)),
                        (format!("{}::~{}()", old_classname, old_classname), format!("{}::~{}()", new_classname, new_classname)),
                        (format!("{}::", old_classname), format!("{}::", new_classname)),
                    ]
                );
                std::fs::rename(
                    old_class_cpp_path.clone(),
                    class_cpp_path.clone()
                ).with_context(|| format!("Tried renaming {} to {}", old_class_cpp_path.display(), class_cpp_path.display())).unwrap_or_else(|_| ());
                
                
                // Rename class in the respective header file
                let old_header_path = file.path().join(format!("{}.h", renameargs.old_modulename));
                let header_path = file.path().join(format!("{}.h", renameargs.new_modulename));
                rename_file_contents(
                    old_header_path.clone(),
                    vec![
                        (format!("{}_CLASS_H", old_uppercase), format!("{}_CLASS_H", new_uppercase)),
                        (format!("class {}", old_classname), format!("class {}", new_classname)),
                        (format!("GDCLASS({}", old_classname), format!("GDCLASS({}", new_classname)),
                        (format!("{}();", old_classname), format!("{}();", new_classname)),
                    ]
                );
                std::fs::rename(
                    old_header_path.clone(),
                    header_path.clone()
                ).with_context(|| format!("Tried renaming {} to {}", old_header_path.display(), header_path.display())).unwrap_or_else(|_| ());


                // Rename class in the register_types.h
                let register_h_path = file.path().join("register_types.h");
                rename_file_contents(
                    register_h_path.clone(),
                    vec![
                        (format!("{}_REGISTER_TYPES_H", old_uppercase), format!("{}_REGISTER_TYPES_H", new_uppercase)),
                        (format!("initialize_{}_module", renameargs.old_modulename), format!("initialize_{}_module", renameargs.new_modulename)),
                    ]
                );

                
                // Rename class in the register_types.cpp
                let register_cpp_path = file.path().join("register_types.cpp");
                rename_file_contents(
                    register_cpp_path.clone(),
                    vec![
                        (format!("#include \"{}.h\"", renameargs.old_modulename), format!("#include \"{}.h\"", renameargs.new_modulename)),
                        (format!("initialize_{}_module", renameargs.old_modulename), format!("initialize_{}_module", renameargs.new_modulename)),
                        (format!("ClassDB::register_class<{}>();", old_classname), format!("ClassDB::register_class<{}>();", new_classname)),
                        (format!("{}_library_init", renameargs.old_modulename), format!("{}_library_init", renameargs.new_modulename)),
                    ]
                );
            },
            _ => (),
        };
    };

    if !renameargs.nobuild {
        let basecmd = get_basecommand();
        let basecmd = (basecmd.0.as_str(), basecmd.1.as_str());
        info!("Running 'scons'");
        let output = Command::new(basecmd.0).arg(basecmd.1)
            .current_dir(pathstr)
            .arg("scons").output();
        _ = print_output(output.unwrap());
    };

    Ok(())
}
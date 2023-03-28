pub mod cli;
use anyhow::{Result, Error};
use log::debug;

pub use cli::*;
pub mod codegen;

mod init;
pub use init::init;

mod rename;
pub use rename::rename;

mod add;
pub use add::add_extension;


pub fn print_output(output: std::process::Output) -> Result<(), Error> {
    if !output.stdout.is_empty() {
        debug!("{}", String::from_utf8(output.stdout)?);
    };
    if !output.stderr.is_empty() {
        debug!("{}", String::from_utf8(output.stderr)?);
    };
    Ok(())
}

pub fn get_basecommand() -> (String, String) {
    if cfg!(target_family = "windows") {
        ("cmd".to_string(), "/C".to_string())
    } else if cfg!(target_family = "unix") {
        ("sh".to_string(), "-c".to_string())
    } else { // WASM - also unix??
        ("sh".to_string(), "-c".to_string())
    }
}
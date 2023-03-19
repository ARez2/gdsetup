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
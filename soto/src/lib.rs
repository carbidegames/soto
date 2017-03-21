extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate walkdir;
#[macro_use] extern crate slog;

mod build;
mod error;
mod files;
pub mod task;

pub use build::build;
pub use error::Error;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

pub fn read_toml<P: Deserialize>(path: &Path) -> Result<P, Error> {
    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(toml::from_str(&data)?)
}

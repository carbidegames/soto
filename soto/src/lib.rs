extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate toml;

mod error;

pub use error::SotoError;

use std::path::PathBuf;
use std::fs::File;
use std::io::{self, Read};

use serde::Deserialize;

/// Build a project in a directory.
pub fn build<P: Into<PathBuf>>(directory: P) -> Result<(), SotoError> {
    let directory = directory.into();

    // Open up the project files
    let soto_proj: SotoProjectFile = read_required(&directory, "SoTo.toml")?;
    let soto_local: SotoLocalFile = read_required(&directory, "SoTo.Local.toml")?;

    println!("{:?}", soto_proj);
    println!("{:?}", soto_local);

    Ok(())
}

fn read_required<P: Deserialize>(directory: &PathBuf, file_name: &str) -> Result<P, SotoError> {
    let mut file = file_in(directory, file_name)
        .map_err(|e| SotoError::RequiredFileReadError(file_name.into(), Box::new(e)))?;
    let mut data = String::new();
    file.read_to_string(&mut data)
        .map_err(|e| SotoError::RequiredFileReadError(file_name.into(), Box::new(e)))?;
    toml::from_str(&data)
        .map_err(|e| SotoError::RequiredFileReadError(file_name.into(), Box::new(e)))
}

fn file_in(path: &PathBuf, file: &str) -> Result<File, io::Error> {
    let mut path = path.clone();
    path.push(file);
    File::open(path)
}

#[derive(Deserialize, Debug)]
struct SotoProjectFile {
    project: SotoProjectFileProject
}

#[derive(Deserialize, Debug)]
struct SotoProjectFileProject {
    prefix: String,
}

#[derive(Deserialize, Debug)]
struct SotoLocalFile {
    game: SotoLocalFileGame,
}

#[derive(Deserialize, Debug)]
struct SotoLocalFileGame {
    bin: String,
    content: String,
}

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate toml;
extern crate walkdir;
#[macro_use] extern crate slog;

mod error;

pub use error::SotoError;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Read};

use serde::Deserialize;
use walkdir::WalkDir;
use slog::Logger;

/// Build a project in a directory.
pub fn build<P: Into<PathBuf>>(log: &Logger, directory: P) -> Result<(), SotoError> {
    let directory = directory.into();

    // Open up the project files
    let _soto_proj: SotoProjectFile = read_required(&directory, "SoTo.toml")?;
    let _soto_local: SotoLocalFile = read_required(&directory, "SoTo.Local.toml")?;

    // Walk the directory looking for files we need to do stuff to
    for entry in WalkDir::new(directory) {
        let entry = entry.unwrap();

        // If it's not a file, skip it
        if !entry.file_type().is_file() { continue; }

        // Handle any extensions we want to look at
        match entry.path().extension().map(|v| v.to_str().unwrap()) {
            Some("toml") => handle_toml(log, entry.path())?,
            _ => ()
        }
    }

    Ok(())
}

fn handle_toml(log: &Logger, path: &Path) -> Result<(), SotoError> {
    // Set up the logger for this file
    let log_path = format!("{}", path.display());
    let log = log.new(o!("file" => log_path));

    // Check the file for a soto tag so we know if we should process it
    let data: SotoTaskFile = read_toml(path)?;
    let data = match data.soto {
        Some(v) => v,
        None => { debug!(log, "No soto tag, skipping"); return Ok(());}
    };

    // We've got a file we want to process, log what we're about to do
    info!(log, "Processing with runner \"{}\"", data.runner);

    Ok(())
}

fn read_required<P: Deserialize>(directory: &PathBuf, file_name: &str) -> Result<P, SotoError> {
    read_toml(&file_in(directory, file_name))
        .map_err(|e| SotoError::RequiredFileReadError(file_name.into(), Box::new(e)))
}

fn read_toml<P: Deserialize>(path: &Path) -> Result<P, SotoError> {
    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(toml::from_str(&data)?)
}

fn file_in(path: &PathBuf, file: &str) -> PathBuf {
    let mut path = path.clone();
    path.push(file);
    path
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

#[derive(Deserialize, Debug)]
struct SotoTaskFile {
    soto: Option<SotoTaskFileSoto>
}

#[derive(Deserialize, Debug)]
struct SotoTaskFileSoto {
    runner: String
}

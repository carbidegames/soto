use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Read};

use serde::Deserialize;
use toml;
use walkdir::WalkDir;
use slog::Logger;

use files::{SotoProjectFile, SotoLocalFile, SotoTaskFile};
use Error;

/// Build a project in a directory.
pub fn build<P: Into<PathBuf>>(log: &Logger, directory: P) -> Result<(), Error> {
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

fn handle_toml(log: &Logger, path: &Path) -> Result<(), Error> {
    // Set up the logger for this file
    let log_path = format!("{}", path.display());
    let log = log.new(o!("file" => log_path));

    // Check the file for a soto tag so we know if we should process it
    let data: SotoTaskFile = read_toml(path)?;
    let data = match data.soto {
        Some(v) => v,
        None => { debug!(log, "No soto tag, skipping"); return Ok(()); }
    };

    // We've got a file we want to process, now go process it
    // TODO: Support receiving logging message from the task while it's running
    info!(log, "Processing with runner \"{}\"", data.runner);
    /*let result = Task {
        runner: data.runner,
        task_file: path.to_path_buf(),
    }.run();*/

    // TODO: Handle result

    Ok(())
}

fn read_required<P: Deserialize>(directory: &PathBuf, file_name: &str) -> Result<P, Error> {
    read_toml(&file_in(directory, file_name))
        .map_err(|e| Error::RequiredFileReadError(file_name.into(), Box::new(e)))
}

fn read_toml<P: Deserialize>(path: &Path) -> Result<P, Error> {
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

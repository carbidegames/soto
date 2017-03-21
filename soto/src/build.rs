use std::path::{Path, PathBuf};

use serde::Deserialize;
use walkdir::WalkDir;
use slog::Logger;

use files::{SotoProjectFile, SotoLocalFile, SotoTaskFile};
use task::{Task};
use {read_toml, Error};

/// Build a project in a directory.
pub fn build<P: Into<PathBuf>>(log: &Logger, directory: P) -> Result<(), Error> {
    let directory = directory.into();

    // Open up the project files
    let project: SotoProjectFile = read_required(&directory, "SoTo.toml")?;
    let local: SotoLocalFile = read_required(&directory, "SoTo.Local.toml")?;

    // Walk the directory looking for files we need to do stuff to
    for entry in WalkDir::new(directory) {
        let entry = entry.unwrap();

        // If it's not a file, skip it
        if !entry.file_type().is_file() { continue; }

        // Handle any extensions we want to look at
        match entry.path().extension().map(|v| v.to_str().unwrap()) {
            Some("toml") => handle_toml(log, entry.path(), &project, &local)?,
            _ => ()
        }
    }

    Ok(())
}

fn handle_toml(
    log: &Logger, path: &Path, project: &SotoProjectFile, local: &SotoLocalFile
) -> Result<(), Error> {
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
    info!(log, "Processing with runner \"{}\"", data.runner);
    let result = Task {
        runner: data.runner,
        task_file: path.to_path_buf(),
    }.run(&log, project.clone(), local.clone());

    // Log the actual result
    match result {
        Ok(_) => info!(log, "Completed successfully"),
        Err(Error::Task(e)) => error!(log, "Error while running task: {}", e),
        Err(e) => return Err(e),
    }

    Ok(())
}

fn read_required<P: Deserialize>(directory: &PathBuf, file_name: &str) -> Result<P, Error> {
    read_toml(&file_in(directory, file_name))
        .map_err(|e| Error::RequiredFileRead(file_name.into(), Box::new(e)))
}

fn file_in(path: &PathBuf, file: &str) -> PathBuf {
    let mut path = path.clone();
    path.push(file);
    path
}

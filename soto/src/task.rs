use std::fmt::Display;
use std::env;
use std::process::Command;
use std::path::PathBuf;
use serde_json;
use Error;

/// Describes a task to be run.
pub struct Task {
    pub runner: String,
    pub task_file: PathBuf,
}

impl Task {
    pub fn run(self) -> Result<(), Error> {
        // TODO: Support receiving logging message from the task while it's running

        // Create the task paramters which we need to send over
        let task_params = TaskParameters {
            working_dir: "./target/working".into(),
            target_dir: "./target/dist".into(),
            target_toml: self.task_file.clone(),
        };
        let task_params_json = serde_json::to_string(&task_params).unwrap();

        // Create the directories this task is going to need
        ::std::fs::create_dir_all(task_params.working_dir)?;
        ::std::fs::create_dir_all(task_params.target_dir)?;

        // Run the actual command
        let result = Command::new(&self.runner)
            .args(&[task_params_json])
            .output()
            .map_err(|e| {
                if let ::std::io::ErrorKind::NotFound = e.kind() {
                    Error::Task(format!(
                        "Can't find command \"{}\", you may have to install it", self.runner
                    ))
                } else {
                    Error::Io(e)
                }
            })?;

        // Check what the result was
        let result_str = ::std::str::from_utf8(&result.stdout).unwrap();
        let err_str = ::std::str::from_utf8(&result.stderr).unwrap();
        let result: TaskResult = serde_json::from_str(result_str)
            .map_err(|e| Error::Task(format!("Result parse error: \"{}\"\nThis may not be a soto task runner or an internal error occurred.\nStdout:\n{}Stderr:\n{}", e, result_str, err_str)))?;

        // If we got an error, return that as an error
        if let Some(error) = result.error {
            return Err(Error::Task(error));
        }

        Ok(())
    }
}

/// Turns the binary into a soto task, parses parameters and serializes result.
pub fn task_wrapper<E: Display, F: FnOnce(TaskParameters) -> Result<(), E>>(task: F) {
    // Get the json from the arguments and turn it into a parameters structure
    let mut args = env::args();
    if args.len() != 2 {
        println!("This is a soto command, do not run this directly!");
        println!("If you need to run this as part of another tool, use the soto library.");
        return;
    }
    let json = args.nth(1).unwrap();
    let params: TaskParameters = serde_json::from_str(&json).unwrap();

    // Run the task itself
    let result = task(params);

    // Turn the task's result into a TaskResult
    let result = match result {
        Ok(_) => TaskResult { error: None },
        Err(e) => TaskResult { error: Some(format!("{}", e)) }
    };

    // Print the result, so the caller can do something with it
    println!("{}", serde_json::to_string(&result).unwrap());
}

#[derive(Serialize, Deserialize)]
pub struct TaskParameters {
    pub working_dir: PathBuf,
    pub target_dir: PathBuf,
    pub target_toml: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct TaskResult {
    pub error: Option<String>,
}

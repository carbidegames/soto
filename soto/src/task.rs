use std::fmt::Display;
use std::env;
use std::process::{Stdio, Command, Child};
use std::path::PathBuf;
use std::io::{BufReader, BufRead};
use serde_json;
use slog::Logger;

use files::{SotoProjectFile, SotoLocalFile};
use Error;

/// Describes a task to be run.
pub struct Task {
    pub runner: String,
    pub task_file: PathBuf,
}

impl Task {
    pub fn run(
        self, log: &Logger, project: SotoProjectFile, local: SotoLocalFile
    ) -> Result<(), Error> {
        // TODO: Support receiving logging message from the task while it's running

        // Create the task paramters which we need to send over
        let task_params = TaskParameters {
            working_dir: "./target/working".into(),
            target_dir: "./target/dist".into(),
            target_toml: self.task_file.clone(),

            project: project,
            local: local,
        };
        let task_params_json = serde_json::to_string(&task_params).unwrap();

        // Create the directories this task is going to need
        ::std::fs::create_dir_all(task_params.working_dir)?;
        ::std::fs::create_dir_all(task_params.target_dir)?;

        // Run the actual command
        let child = Command::new(&self.runner)
            .args(&[task_params_json])
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| {
                if let ::std::io::ErrorKind::NotFound = e.kind() {
                    Error::Task(format!(
                        "Can't find command \"{}\", you may have to install it", self.runner
                    ))
                } else {
                    Error::Io(e)
                }
            })?;
        let mut child = WaitGuard {child: child};

        // Loop read the messages we get back
        let mut child_out = BufReader::new(child.child.stdout.as_mut().unwrap());
        let mut message = String::new();
        loop {
            child_out.read_line(&mut message)?;

            // If the message is empty, the child unexpectedly closed
            if message.len() == 0 {
                return Err(Error::Task("Runner unexpectedly closed, this may not be a soto task runner or an internal error occurred.".into()));
            }

            // If it's not, try to parse the json we received
            let result: TaskMessage = serde_json::from_str(&message)
                .map_err(|e| {
                    error!(log, "An error has occurred, waiting for runner to close...");
                    Error::Task(format!(
                        "Message parse error: \"{}\"\nThis may not be a soto task runner or an internal error occurred.\nMessage:\n{}", e, message
                    ))
                })?;

            // We have a valid result, see what it says
            match result {
                // A log message just needs to be passed through
                TaskMessage::Log(text) => debug!(log, text),
                // A result means this task is done
                TaskMessage::Result(res) => {
                    if let Some(error) = res.error {
                        error!(log, error);
                    }
                    break;
                }
            }

            // We aren't done yet, clear the previous message
            message.clear();
        }

        Ok(())
    }
}

struct WaitGuard {
    child: Child,
}

impl Drop for WaitGuard {
    fn drop(&mut self) {
        let _ = self.child.wait();
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
    let message = TaskMessage::Result(result);
    println!("{}", serde_json::to_string(&message).unwrap());
}

/// Relays a debug logging message from a task back to soto.
pub fn task_log<S: ToString>(text: S) {
    let message = TaskMessage::Log(text.to_string());
    println!("{}", serde_json::to_string(&message).unwrap());
}

#[derive(Serialize, Deserialize)]
pub struct TaskParameters {
    pub working_dir: PathBuf,
    pub target_dir: PathBuf,
    pub target_toml: PathBuf,

    pub project: SotoProjectFile,
    pub local: SotoLocalFile,
}

#[derive(Serialize, Deserialize)]
pub struct TaskResult {
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum TaskMessage {
    Log(String),
    Result(TaskResult),
}

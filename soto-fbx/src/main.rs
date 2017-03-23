extern crate cgmath;
#[macro_use] extern crate serde_derive;
extern crate soto;
extern crate sotolib_fbx;
extern crate sotolib_smd;

mod qc;
mod smd;
mod task;

use std::path::PathBuf;

use soto::task::{task_wrapper, TaskParameters};
use soto::Error;

use task::SotoFbxTask;

fn main() {
    // This is a soto task, so we need to run the wrapper
    task_wrapper(task_main);
}

fn task_main(params: TaskParameters) -> Result<(), Error> {
    // First, read in the toml we got told to read
    let toml: SotoFbxTask = soto::read_toml(&params.target_toml)?;

    // Generate the needed SMDs
    let mut reference_smd = params.working_dir.clone();
    reference_smd.push("reference.smd");
    smd::create_reference_smd(&PathBuf::from(&toml.model.reference), &reference_smd)?;

    // Generate the QC
    let mut target_qc = params.working_dir.clone();
    target_qc.push("script.qc");
    qc::generate_qc(&target_qc, &toml, "reference.smd")?;

    // Finally, run the model build
    qc::build_qc(&target_qc, &params, &params.local.game.content)?;

    Ok(())
}

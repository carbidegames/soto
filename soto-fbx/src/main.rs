extern crate cgmath;
#[macro_use] extern crate serde_derive;
extern crate soto;
extern crate sotolib_fbx;
extern crate sotolib_smd;

mod qc;
mod smd;
mod task;

use std::fs::File;

use soto::task::{task_wrapper, TaskParameters, task_log};
use soto::{Error};
use sotolib_smd::{SmdExportExt};

use task::SotoFbxTask;

fn main() {
    // This is a soto task, so we need to run the wrapper
    task_wrapper(task_main);
}

fn task_main(params: TaskParameters) -> Result<(), Error> {
    // First, read in the toml we got told to read
    let toml: SotoFbxTask = soto::read_toml(&params.target_toml)?;

    // Generate the reference SDM
    let reference_smd = smd::create_reference_smd(&toml.model.reference)?;

    // Export the reference SMD
    let mut reference_smd_file = params.working_dir.clone();
    reference_smd_file.push("reference.smd");
    let export_file = File::create(reference_smd_file)?;
    reference_smd.export(export_file).unwrap();

    // Generate the animation SDMs
    for sequence in &toml.sequences {
        task_log(format!("Generating animation \"{}\"...", sequence.0));

        // Generate the SMD
        let animation_smd = smd::create_animation_smd(
            &reference_smd, &sequence.1.file
        )?;

        // Export the SMD
        let mut animation_smd_file = params.working_dir.clone();
        animation_smd_file.push(format!("animation_{}.smd", sequence.0));
        let export_file = File::create(animation_smd_file)?;
        animation_smd.export(export_file).unwrap();
    }

    // Generate the QC
    let mut target_qc = params.working_dir.clone();
    target_qc.push("script.qc");
    qc::generate_qc(&target_qc, &toml, "reference.smd")?;

    // Finally, run the model build
    qc::build_qc(&target_qc, &params, &params.local.game.content)?;

    Ok(())
}

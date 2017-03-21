extern crate soto;
extern crate sotolib_fbx;
extern crate sotolib_smd;

use std::fs::File;
use std::io::BufReader;

use soto::task::{task_wrapper, TaskParameters, TaskResult};
use sotolib_fbx::{RawFbx, SimpleFbx, FbxObject, friendly_name, id_name};
use sotolib_smd::{Smd, SmdVertex, SmdLink, SmdTriangle, SmdExportExt};

fn main() {
    // This is a soto task, so we need to run the wrapper
    task_wrapper(task_main);

    // Read in the fbx
    /*let file = BufReader::new(File::open("../debugref/test_cube.fbx").unwrap());
    let fbx = SimpleFbx::from_raw(&RawFbx::parse(file).unwrap());

    // Create a target SMD to export to
    // TODO: Multiple export SMDs for animations (idle & user-specified)
    let mut smd = Smd::new();

    // Go over all model objects
    for obj in &fbx.objects {
        if let &FbxObject::Model(ref model) = obj.1 {
            // We've found a model, log that we're found it
            println!("Found model \"{}\"", friendly_name(&model.name));

            // For this model object, find the linked geometry
            for obj in fbx.children_of(model.id) {
                if let &FbxObject::Geometry(ref geom) = obj {
                    // We've got geometry, first get a new bone from the SMD for us to attach to
                    let bone_id = smd.new_bone(&id_name(&model.name).unwrap()).unwrap();

                    // Add the actual triangles
                    let tris = geom.triangles();
                    for tri in tris {
                        // Turn the vertices in this triangle to SMD vertices
                        let mut smd_verts: [SmdVertex; 3] = Default::default();
                        for (i, vert) in tri.iter().enumerate() {
                            smd_verts[i] = SmdVertex {
                                parent_bone: 0, // This is overwritten by links
                                position: vert.0,
                                normal: vert.1,
                                uv: vert.2,
                                links: vec!(
                                    SmdLink {
                                        bone: bone_id,
                                        weight: 1.0,
                                    }
                                )
                            };
                        }

                        // Add the actual SMD triangle
                        smd.triangles.push(SmdTriangle {
                            material: "debug/debugempty".into(),
                            vertices: smd_verts,
                        });
                    }
                }
            }
        }
    }

    // Export the SMD
    let export_file = File::create("./test.smd").unwrap();
    smd.export(export_file).unwrap();*/
}

fn task_main(_params: TaskParameters) -> TaskResult {
    TaskResult {
        error: Some("nothing to do".into())
    }
}

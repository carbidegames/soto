extern crate fbx_simple;

use std::fs::File;
use std::io::BufReader;

use fbx_simple::{RawFbx, SimpleFbx, FbxObject, friendly_name};

fn main() {
    // TODO: Read in a convert config definition that contains all the values needed for converting
    // This tool should convert various FBX files defined in an input descriptor file into a MDL

    // Read in the fbx
    let file = BufReader::new(File::open("../debugref/test_cube.fbx").unwrap());
    let fbx = SimpleFbx::from_raw(&RawFbx::parse(file).unwrap());

    // Go over all model objects
    for obj in &fbx.objects {
        if let &FbxObject::Model(ref model) = obj.1 {
            // We've found a model, log that we're found it
            println!("Found model \"{}\"", friendly_name(&model.name));

            // For this model object, find the linked geometry
            for obj in fbx.children_of(model.id) {
                if let &FbxObject::Geometry(ref geom) = obj {
                    // We've got geometry, add it to the SMD
                    let tris = geom.triangles();
                    println!(" Triangles: {:?}", tris);
                }
            }
        }
    }

    // TODO: Actually convert
}

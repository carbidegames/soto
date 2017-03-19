extern crate fbx_simple;

use std::fs::File;
use std::io::BufReader;

use fbx_simple::{RawFbx, SimpleFbx};

fn main() {
    // TODO: Read in a convert config definition that contains all the values needed for converting

    // Read in the fbx
    let file = BufReader::new(File::open("./debugref/test_cube.fbx").unwrap());
    let fbx = SimpleFbx::from_raw(&RawFbx::parse(file).unwrap());

    // Debug print everything we have
    println!("{:?}", fbx);

    // TODO: Actually convert
}

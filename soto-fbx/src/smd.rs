use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;

use cgmath::{Matrix4, Deg, Rad, Vector4};
use soto::task::{task_log};
use soto::Error;
use sotolib_fbx::{RawFbx, SimpleFbx, FbxObject, id_name, friendly_name, FbxGeometry, FbxModel};
use sotolib_smd::{Smd, SmdVertex, SmdLink, SmdTriangle, SmdExportExt, SmdAnimationFrameBone, BoneId};

pub fn create_reference_smd(fbx: &PathBuf, target_smd: &PathBuf) -> Result<(), Error> {
    // Read in the fbx we got told to convert
    let file = BufReader::new(File::open(&fbx).unwrap());
    let fbx = SimpleFbx::from_raw(&RawFbx::parse(file).unwrap());

    // Create a target SMD to export to
    // TODO: Multiple export SMDs for animations (idle & user-specified)
    let mut smd = Smd::new();

    // Go over all model objects
    for obj in &fbx.objects {
        if let &FbxObject::Model(ref model) = obj.1 {
            add_model_to_smd(&mut smd, &model, &fbx);
        }
    }

    // Export the SMD
    let export_file = File::create(target_smd)?;
    smd.export(export_file).unwrap();

    Ok(())
}

fn add_model_to_smd(smd: &mut Smd, model: &FbxModel, fbx: &SimpleFbx) {
    // We've found a model, log that we're found it
    task_log(format!("Adding model \"{}\" to SMD data", friendly_name(&model.name)));

    // Create a bone for this model
    let bone_id = smd.new_bone(&id_name(&model.name).unwrap()).unwrap();

    // Insert the default animation state into the SMD as 0th frame
    let bone_anim = SmdAnimationFrameBone {
        translation: model.translation,
        rotation: [
            Rad::from(Deg(model.rotation[0])).0,
            Rad::from(Deg(model.rotation[1])).0,
            Rad::from(Deg(model.rotation[2])).0,
        ],
    };
    smd.set_animation(0, bone_id, bone_anim);

    // Create a multiplication matrix, we need this because SMD expects the vertices to be
    // multiplied in advance for the idle frame.
    let matrix =
        Matrix4::from_translation(model.translation.into()) *
        Matrix4::from_angle_z(Deg(model.rotation[2])) *
        Matrix4::from_angle_y(Deg(model.rotation[1])) *
        Matrix4::from_angle_x(Deg(model.rotation[0]));

    // For this model object, find the linked geometry and add the triangles
    for obj in fbx.children_of(model.id) {
        if let &FbxObject::Geometry(ref geometry) = obj {
            add_geometry_triangles_to_smd(smd, geometry, &matrix, bone_id);
        }
    }
}

fn add_geometry_triangles_to_smd(
    smd: &mut Smd, geometry: &FbxGeometry, matrix: &Matrix4<f32>, bone_id: BoneId
) {
    // Add the actual triangles
    let tris = geometry.triangles();
    for tri in tris {
        // Turn the vertices in this triangle to SMD vertices
        let mut smd_verts: [SmdVertex; 3] = Default::default();
        for (i, vert) in tri.iter().enumerate() {
            // Multiply the vectors that need to be multiplied
            let pos = matrix * Vector4::new(vert.0[0], vert.0[1], vert.0[2], 1.0);
            let norm = matrix * Vector4::new(vert.1[0], vert.1[1], vert.1[2], 0.0);

            smd_verts[i] = SmdVertex {
                parent_bone: 0, // This is overwritten by links
                position: pos.truncate().into(),
                normal: norm.truncate().into(),
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
            material: "layl_test_texture".into(),
            vertices: smd_verts,
        });
    }
}

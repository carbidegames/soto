use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;

use cgmath::{Matrix4, Deg, Rad, Vector4, SquareMatrix, Vector3};
use soto::task::{task_log};
use soto::Error;
use sotolib_fbx::{RawFbx, SimpleFbx, FbxObject, id_name, friendly_name, FbxObjectTreeNode};
use sotolib_smd::{Smd, SmdVertex, SmdTriangle, SmdExportExt, SmdAnimationFrameBone, SmdBone};

pub fn create_reference_smd(fbx: &PathBuf, target_smd: &PathBuf) -> Result<(), Error> {
    // Read in the fbx we got told to convert
    let file = BufReader::new(File::open(&fbx).unwrap());
    let fbx = SimpleFbx::from_raw(&RawFbx::parse(file).unwrap());
    let fbx_tree = FbxObjectTreeNode::from_simple(&fbx);

    // Go over all FBX root nodes and turn them into SMD data
    let mut smd = Smd::new();
    let matrix = Matrix4::identity();
    process_fbx_node(&fbx_tree, &matrix, &mut smd, None)?;

    // Export the SMD
    let export_file = File::create(target_smd)?;
    smd.export(export_file).unwrap();

    Ok(())
}

fn process_fbx_node(
    fbx_node: &FbxObjectTreeNode, matrix: &Matrix4<f32>,
    mut smd: &mut Smd, current_bone: Option<&SmdBone>,
) -> Result<(), Error> {
    // Perform node type specific information
    match fbx_node.object {
        FbxObject::Geometry(ref geometry) => {
            // Add triangles to parent node
            let tris = geometry.triangles();
            for tri in tris {
                // Turn the vertices in this triangle to SMD vertices
                let mut smd_verts: [SmdVertex; 3] = Default::default();
                for (i, vert) in tri.iter().enumerate() {
                    // Multiply the vectors that need to be multiplied
                    let pos = matrix * Vector4::new(vert.0[0], vert.0[1], vert.0[2], 1.0);
                    let norm = matrix * Vector4::new(vert.1[0], vert.1[1], vert.1[2], 0.0);

                    smd_verts[i] = SmdVertex {
                        parent_bone: current_bone.unwrap().id, // This is overwritten by links
                        position: pos.truncate().into(),
                        normal: norm.truncate().into(),
                        uv: vert.2,
                        links: vec!(
                            /*Not needed, we aren't using weights anyways so this done by parent_bone
                            SmdLink {
                                bone: bone_id,
                                weight: 1.0,
                            }*/
                        )
                    };
                }

                // Add the actual SMD triangle
                smd.triangles.push(SmdTriangle {
                    material: "layl_test_texture".into(),
                    vertices: smd_verts,
                });
            }
        },
        FbxObject::Model(ref model) => {
            task_log(format!("Adding model \"{}\" to SMD data", friendly_name(&model.name)));

            // Create a new bone and set the transformations
            let new_bone = smd.new_bone(&id_name(&model.name).unwrap(), current_bone.map(|b| b.id))
                .ok_or_else(|| Error::Task(format!("Bone \"{}\" exists multiple times in the FBX", &model.name)))?
                .clone(); // Clone needed to avoid a borrow since we need to mut borrow later
            let first_frame = SmdAnimationFrameBone {
                translation: model.translation,
                rotation: [
                    Rad::from(Deg(model.rotation[0])).0,
                    Rad::from(Deg(model.rotation[1])).0,
                    Rad::from(Deg(model.rotation[2])).0,
                ],
            };
            smd.set_animation(0, new_bone.id, first_frame);

            // Create a new transformation matrix for child nodes
            let rot_pivot: Vector3<_> = model.rotation_pivot.into();
            let rot_pivot_mat = Matrix4::from_translation(rot_pivot);
            let matrix =
                matrix *
                Matrix4::from_translation(model.translation.into()) *
                rot_pivot_mat *
                Matrix4::from_angle_z(Deg(model.rotation[2])) *
                Matrix4::from_angle_y(Deg(model.rotation[1])) *
                Matrix4::from_angle_x(Deg(model.rotation[0])) *
                rot_pivot_mat.invert().unwrap() * // This may need to be rotated in reverse
                Matrix4::from_nonuniform_scale(model.scale[0], model.scale[1], model.scale[2]);

            // Make sure the child nodes will receive this new bone
            for node in &fbx_node.nodes {
                process_fbx_node(node, &matrix, smd, Some(&new_bone))?;
            }
        },
        FbxObject::Root | FbxObject::NotSupported(_) => {
            // Just go straight to the children
            for node in &fbx_node.nodes {
                process_fbx_node(node, matrix, smd, current_bone)?;
            }
        }
    }

    Ok(())
}

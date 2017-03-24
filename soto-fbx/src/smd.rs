use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;

use cgmath::{Matrix4, Deg, Rad, Vector4, SquareMatrix, Vector3};
use soto::task::{task_log};
use soto::Error;
use sotolib_fbx::{RawFbx, id_name, friendly_name, ObjectTreeNode};
use sotolib_fbx::simple::{SimpleFbx, ObjectType, ModelProperties};
use sotolib_smd::{Smd, SmdVertex, SmdTriangle, SmdExportExt, SmdAnimationFrameBone, SmdBone};

pub fn create_reference_smd(fbx: &PathBuf, target_smd: &PathBuf) -> Result<(), Error> {
    // Read in the fbx we got told to convert
    let file = BufReader::new(File::open(&fbx).unwrap());
    let fbx = SimpleFbx::from_raw(&RawFbx::parse(file).unwrap());
    let fbx_tree = ObjectTreeNode::from_simple(&fbx);

    // Go over all FBX root nodes and turn them into SMD data
    let mut smd = Smd::new();
    process_fbx_node(&fbx_tree, &Matrix4::identity(), Vector3::new(0.0, 0.0, 0.0), &mut smd, None)?;

    // Export the SMD
    let export_file = File::create(target_smd)?;
    smd.export(export_file).unwrap();

    Ok(())
}

fn process_fbx_node(
    fbx_node: &ObjectTreeNode, matrix: &Matrix4<f32>, pivot: Vector3<f32>,
    mut smd: &mut Smd, current_bone: Option<&SmdBone>,
) -> Result<(), Error> {
    // Perform node type specific information
    match fbx_node.object.class {
        ObjectType::Geometry(ref geometry) => {
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
        ObjectType::Model(ref _model) => {
            task_log(format!("Adding model \"{}\" to SMD data", friendly_name(&fbx_node.object.name)));
            let properties = ModelProperties::from_generic(&fbx_node.object.properties);

            // Create a new transformation matrix
            let rot_pivot: Vector3<_> = properties.rotation_pivot.into();
            let rot_pivot_mat = Matrix4::from_translation(rot_pivot);

            let rotation =
                Matrix4::from_angle_z(Deg(properties.rotation[2])) *
                Matrix4::from_angle_y(Deg(properties.rotation[1])) *
                Matrix4::from_angle_x(Deg(properties.rotation[0]));

            let local_matrix_for_vertices =
                Matrix4::from_translation(properties.translation.into()) *
                rot_pivot_mat *
                rotation *
                rot_pivot_mat.invert().unwrap() *
                Matrix4::from_nonuniform_scale(
                    properties.scale[0],
                    properties.scale[1],
                    properties.scale[2]
                );

            // Create a new bone and set the transformations
            let new_bone = smd.new_bone(
                &id_name(&fbx_node.object.name).unwrap(),
                current_bone.map(|b| b.id)
            )
                .ok_or_else(|| Error::Task(format!(
                    "Bone \"{}\" exists multiple times in the FBX",
                    &fbx_node.object.name
                )))?
                .clone(); // Clone needed to avoid a borrow since we need to mut borrow SMD later
            let first_frame = SmdAnimationFrameBone {
                // This needs to be derived from the matrix to get the right location
                translation: (
                    Matrix4::from_translation(-pivot) *
                    local_matrix_for_vertices *
                    Vector4::new(rot_pivot.x, rot_pivot.y, rot_pivot.z, 1.0)
                ).truncate().into(),
                rotation: [
                    Rad::from(Deg(properties.rotation[0])).0,
                    Rad::from(Deg(properties.rotation[1])).0,
                    Rad::from(Deg(properties.rotation[2])).0,
                ],
            };
            smd.set_animation(0, new_bone.id, first_frame);

            // Make new matrices for children
            let matrix = matrix * local_matrix_for_vertices;
            let pivot = rot_pivot;

            // Make sure the child nodes will receive this new bone
            for node in &fbx_node.nodes {
                process_fbx_node(node, &matrix, pivot, smd, Some(&new_bone))?;
            }
        },
        ObjectType::Root | ObjectType::NotSupported(_) => {
            // Just go straight to the children
            for node in &fbx_node.nodes {
                process_fbx_node(node, matrix, pivot, smd, current_bone)?;
            }
        }
    }

    Ok(())
}

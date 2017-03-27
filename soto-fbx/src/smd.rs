use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;

use cgmath::{Matrix4, Deg, Rad, Vector4, SquareMatrix, Vector3};
use soto::task::{task_log};
use soto::Error;
use sotolib_fbx::{RawFbx, id_name, friendly_name, ObjectTreeNode};
use sotolib_fbx::animation::{Animation};
use sotolib_fbx::simple::{Object, SimpleFbx, ObjectType, ModelProperties, Geometry};
use sotolib_smd::{Smd, SmdVertex, SmdTriangle, SmdAnimationFrameBone, SmdBone};

pub fn create_reference_smd(fbx: &PathBuf) -> Result<Smd, Error> {
    // Read in the fbx we got told to convert
    let file = BufReader::new(File::open(&fbx).unwrap());
    let fbx = SimpleFbx::from_raw(&RawFbx::parse(file).unwrap()).unwrap();
    let fbx_tree = ObjectTreeNode::from_simple(&fbx);

    // Go over all FBX root nodes and turn them into SMD data
    let mut smd = Smd::new();
    process_fbx_node(
        &fbx_tree, &mut smd,
        &Matrix4::identity(), Vector3::new(0.0, 0.0, 0.0),
        None
    )?;

    Ok(smd)
}

pub fn create_animation_smd(ref_smd: &Smd, fbx: &PathBuf) -> Result<Smd, Error> {
    // Read in the fbx we got told to convert
    let file = BufReader::new(File::open(&fbx).unwrap());
    let mut fbx = SimpleFbx::from_raw(&RawFbx::parse(file).unwrap()).unwrap();

    // Read in the animation data itself
    let animation = Animation::from_simple(&fbx).unwrap();

    // Finally, turn the animation data into bone positions in the SMD
    let mut smd = Smd::new();
    for frame in 0..animation.frame_count(&fbx) {
        // First transform the FBX for this frame
        animation.transform_fbx_to_frame(&mut fbx, frame);

        // Now go over all models
        for (_, model) in fbx.objects.iter().filter(|&(_, o)| o.class.type_name() == "Model") {
            // For this model, look up the matching BoneId in the reference SMD
            if let Some(bone_id) = ref_smd.id_of_bone(&id_name(&model.name).unwrap()) {
                // Now that we have a model and a bone, we need the current translation and rotation
                // for the model
                let (translation, rotation) = calculate_world_transforms_for(&fbx, model);

                // And now that we have those, finally add the bone data to the animation SMD
                smd.set_animation(frame, bone_id, SmdAnimationFrameBone {
                    translation: translation.into(),
                    rotation: rotation.into(),
                });
            }
        }
    }

    Ok(smd)
}

fn process_fbx_node(
    fbx_node: &ObjectTreeNode, mut smd: &mut Smd,
    matrix: &Matrix4<f32>, pivot: Vector3<f32>,
    current_bone: Option<&SmdBone>,
) -> Result<(), Error> {
    // Perform node type specific information
    match fbx_node.object.class {
        ObjectType::Geometry(ref geometry) =>
            process_geometry(smd, geometry, matrix, current_bone.unwrap()),
        ObjectType::Model(ref _model) =>
            process_model(fbx_node, smd, matrix, pivot, current_bone)?,
        _ => {
            // Just go straight to the children
            for node in &fbx_node.nodes {
                process_fbx_node(node, smd, matrix, pivot, current_bone)?;
            }
        }
    }

    Ok(())
}

fn process_geometry(smd: &mut Smd, geometry: &Geometry, matrix: &Matrix4<f32>, current_bone: &SmdBone) {
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
                parent_bone: current_bone.id, // This is overwritten by links
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
}

fn process_model(
    fbx_node: &ObjectTreeNode, smd: &mut Smd,
    matrix: &Matrix4<f32>, pivot: Vector3<f32>,
    current_bone: Option<&SmdBone>
) -> Result<(), Error> {
    task_log(format!("Adding model \"{}\" to SMD data", friendly_name(&fbx_node.object.name)));
    let properties = ModelProperties::from_generic(&fbx_node.object.properties);

    // Create a new transformation matrix
    let (vertices_matrix, rot_pivot) = local_matrices(&properties);

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
            vertices_matrix *
            Vector4::new(rot_pivot.x, rot_pivot.y, rot_pivot.z, 1.0)
        ).truncate().into(),
        // This can just be directly copied over
        rotation: [
            Rad::from(Deg(properties.rotation[0])).0,
            Rad::from(Deg(properties.rotation[1])).0,
            Rad::from(Deg(properties.rotation[2])).0,
        ],
    };
    smd.set_animation(0, new_bone.id, first_frame);

    // Make new matrices for children
    let matrix = matrix * vertices_matrix;
    let pivot = rot_pivot;

    // Make sure the child nodes will receive this new bone
    for node in &fbx_node.nodes {
        process_fbx_node(node, smd, &matrix, pivot, Some(&new_bone))?;
    }

    Ok(())
}

/// Returns (Translation, Rotation)
fn calculate_world_transforms_for(fbx: &SimpleFbx, obj: &Object) -> (Vector3<f32>, Vector3<f32>) {
    // First get the world matrix and pivot
    let (matrix, pivot, parent_pivot) = world_matrices_of(fbx, obj);

    // Now transform a 0,0,0 vector to get the actual pivot
    let translation = (
        Matrix4::from_translation(-parent_pivot) *
        matrix *
        Vector4::new(pivot.x, pivot.y, pivot.z, 1.0)
    ).truncate().into();

    // This can just be directly copied over
    let properties = ModelProperties::from_generic(&obj.properties);
    let rotation = Vector3::new(
        Rad::from(Deg(properties.rotation[0])).0,
        Rad::from(Deg(properties.rotation[1])).0,
        Rad::from(Deg(properties.rotation[2])).0
    );

    (translation, rotation)
}

fn world_matrices_of(fbx: &SimpleFbx, obj: &Object) -> (Matrix4<f32>, Vector3<f32>, Vector3<f32>) {
    // Start with the local matrix
    let (l_mat, l_piv) = local_matrices(&ModelProperties::from_generic(&obj.properties));

    // Check this obj's parents
    let parent_id = fbx.parent_of(obj.id).unwrap();

    // If this is a root object, we're done, return the data we got
    if parent_id == 0 {
        return (l_mat, l_piv, Vector3::new(0.0, 0.0, 0.0))
    }

    // If this isn't a root object, we need to get that obj's data first
    let (p_mat, p_piv, _) = world_matrices_of(fbx, &fbx.objects[&parent_id]);

    // Then return our data transformed
    (p_mat * l_mat, l_piv, p_piv)
}

/// Returns (vertices_matrix, rot_pivot)
fn local_matrices(properties: &ModelProperties) -> (Matrix4<f32>, Vector3<f32>) {
    let rotation_offset = properties.rotation_offset.into();
    let rotation_offset_mat = Matrix4::from_translation(rotation_offset);
    let rot_pivot: Vector3<_> = properties.rotation_pivot.into();
    let rot_pivot_mat = Matrix4::from_translation(rot_pivot);

    let pre_rotation = euler_rotation_to_matrix(properties.pre_rotation);
    let rotation = euler_rotation_to_matrix(properties.rotation);
    let post_rotation = euler_rotation_to_matrix(properties.post_rotation);

    let scale = Matrix4::from_nonuniform_scale(
        properties.scale[0],
        properties.scale[1],
        properties.scale[2]
    );

    let local_matrix_for_vertices =
        Matrix4::from_translation(properties.translation.into()) *

        // Rotation
        rotation_offset_mat *
        rot_pivot_mat *
        pre_rotation *
        rotation *
        post_rotation *
        rot_pivot_mat.invert().unwrap() *

        // Scale TODO: Complete implementation
        //scale_offset *
        //scale_pivot *
        scale; // *
        //scale_pivot.invert().unwrap();

    (local_matrix_for_vertices, rot_pivot)
}

fn euler_rotation_to_matrix(rot_degs: [f32; 3]) -> Matrix4<f32> {
    Matrix4::from_angle_z(Deg(rot_degs[2])) *
    Matrix4::from_angle_y(Deg(rot_degs[1])) *
    Matrix4::from_angle_x(Deg(rot_degs[0]))
}

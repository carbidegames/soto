mod export;

pub use export::SmdExportExt;

use std::collections::HashMap;

pub type BoneId = u32;

pub struct SmdBone {
    pub id: BoneId,
    pub name: String,
}

pub struct SmdLink {
    pub bone: BoneId,
    pub weight: f32,
}

#[derive(Default)]
pub struct SmdVertex {
    pub parent_bone: BoneId,
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub links: Vec<SmdLink>,
}

pub struct SmdTriangle {
    pub material: String,
    pub vertices: [SmdVertex; 3],
}

pub struct SmdAnimationFrameBone {
    pub translation: [f32; 3],
    pub rotation: [f32; 3],
}

#[derive(Default)]
pub struct SmdAnimationFrame {
    pub bones: HashMap<BoneId, SmdAnimationFrameBone>
}

pub struct Smd {
    pub bones: Vec<SmdBone>,
    pub animation_frames: HashMap<i32, SmdAnimationFrame>,
    pub triangles: Vec<SmdTriangle>,
}

impl Smd {
    pub fn new() -> Self {
        Smd {
            bones: Vec::new(),
            animation_frames: HashMap::new(),
            triangles: Vec::new(),
        }
    }

    pub fn new_bone(&mut self, name: &str) -> Option<u32> {
        // First make sure this bone doesn't already exist
        if self.bones.iter().any(|b| b.name == name) {
            return None;
        }

        // Find a new ID for this bone
        let mut id = 0;
        for bone in &self.bones {
            if bone.id >= id {
                id = bone.id + 1;
            }
        }

        // Add it
        self.bones.push(SmdBone {
            id: id,
            name: name.into(),
        });

        // And finally return the new ID
        Some(id)
    }

    /// Sets the state of a bone at a specific frame, overwriting anything previously there.
    pub fn set_animation(&mut self, frame: i32, bone_id: BoneId, bone: SmdAnimationFrameBone) {
        let frame = self.animation_frames.entry(frame)
            .or_insert_with(|| SmdAnimationFrame::default());
        frame.bones.insert(bone_id, bone);
    }
}

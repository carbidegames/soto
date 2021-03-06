mod export;

pub use export::SmdExportExt;

use std::collections::{BTreeMap};

pub type BoneId = i32;

#[derive(Clone)]
pub struct SmdBone {
    pub id: BoneId,
    pub name: String,
    pub parent: Option<BoneId>,
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
    pub bones: BTreeMap<BoneId, SmdAnimationFrameBone>
}

#[derive(Default)]
pub struct Smd {
    pub bones: Vec<SmdBone>,
    pub animation_frames: BTreeMap<i32, SmdAnimationFrame>,
    pub triangles: Vec<SmdTriangle>,
}

impl Smd {
    pub fn new() -> Self {
        Smd {
            bones: Vec::new(),
            animation_frames: BTreeMap::new(),
            triangles: Vec::new(),
        }
    }

    pub fn new_bone(&mut self, name: &str, parent: Option<BoneId>) -> Option<&SmdBone> {
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
            parent: parent,
        });

        // And finally return the new bone
        Some(&self.bones[self.bones.len()-1])
    }

    pub fn id_of_bone(&self, name: &str) -> Option<BoneId> {
        // Find the bone in the list and return its ID
        self.bones.iter().find(|b| b.name == name).map(|b| b.id)
    }

    /// Sets the state of a bone at a specific frame, overwriting anything previously there.
    pub fn set_animation(&mut self, frame: i32, bone_id: BoneId, bone: SmdAnimationFrameBone) {
        let frame = self.animation_frames.entry(frame)
            .or_insert_with(|| SmdAnimationFrame::default());
        frame.bones.insert(bone_id, bone);
    }
}

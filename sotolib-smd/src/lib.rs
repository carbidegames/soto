mod export;

pub use export::SmdExportExt;

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

pub struct Smd {
    pub bones: Vec<SmdBone>,
    pub triangles: Vec<SmdTriangle>,
}

impl Smd {
    pub fn new() -> Self {
        Smd {
            bones: Vec::new(),
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
}

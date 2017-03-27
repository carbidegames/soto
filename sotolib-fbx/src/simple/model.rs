use simple::{Properties};
use {RawNode};

#[derive(Clone, Debug, Default)]
pub struct Model {
}

impl Model {
    pub fn from_node(_node: &RawNode) -> Self {
        Model {
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModelProperties {
    pub translation: [f32; 3],
    pub pre_rotation: [f32; 3],
    pub rotation: [f32; 3],
    pub post_rotation: [f32; 3],
    pub scale: [f32; 3],
    pub rotation_offset: [f32; 3],
    pub rotation_pivot: [f32; 3],
}

impl ModelProperties {
    pub fn from_generic(properties: &Properties) -> Self {
        // Find the translation in the model
        let mut translation: [f32; 3] = Default::default();
        if let Some(trans) = properties.get("Lcl Translation") {
            translation = trans.to_vector3();
        }

        // Same for rotations
        let mut pre_rotation: [f32; 3] = Default::default();
        if let Some(rot) = properties.get("PreRotation") {
            pre_rotation = rot.to_vector3();
        }
        let mut rotation: [f32; 3] = Default::default();
        if let Some(rot) = properties.get("Lcl Rotation") {
            rotation = rot.to_vector3();
        }
        let mut post_rotation: [f32; 3] = Default::default();
        if let Some(rot) = properties.get("PostRotation") {
            post_rotation = rot.to_vector3();
        }

        // Same for scale
        let mut scale: [f32; 3] = [1.0, 1.0, 1.0];
        if let Some(sca) = properties.get("Lcl Scaling") {
            scale = sca.to_vector3();
        }

        // Same for pivots
        let mut rotation_offset: [f32; 3] = Default::default();
        if let Some(piv) = properties.get("RotationOffset") {
            rotation_offset = piv.to_vector3();
        }
        let mut rotation_pivot: [f32; 3] = Default::default();
        if let Some(piv) = properties.get("RotationPivot") {
            rotation_pivot = piv.to_vector3();
        }

        ModelProperties {
            translation: translation,
            pre_rotation: pre_rotation,
            rotation: rotation,
            post_rotation: post_rotation,
            scale: scale,
            rotation_offset: rotation_offset,
            rotation_pivot: rotation_pivot,
        }
    }
}

#[cfg(test)]
mod tests {
    use fbx_direct::common::OwnedProperty;
    use simple::{Property, Properties};
    use super::*;

    #[test]
    fn it_reads_model_properties() {
        let expected_translation = [32.4, 125.3, -29.1];
        let mut properties = Properties::new();
        properties.insert(
            "Lcl Translation".into(),
            Property {
                name: "Lcl Translation".into(),
                values: expected_translation.iter().map(|v| OwnedProperty::F32(*v)).collect(),
            }
        );

        let properties = ModelProperties::from_generic(&properties);

        assert!(properties.translation == expected_translation);
    }
}

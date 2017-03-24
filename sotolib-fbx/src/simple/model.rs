use std::collections::HashMap;
use fbx_direct::common::OwnedProperty;
use {RawNode};

#[derive(Debug, Clone)]
pub struct Model {
    pub id: i64,
    pub name: String,
    pub translation: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub rotation_pivot: [f32; 3],
}

impl Model {
    pub fn from_node(node: &RawNode) -> Self {
        // Get a map of properties
        let properties = read_properties(node.find_child("Properties70").unwrap());

        // Find the translation in the model
        let mut translation: [f32; 3] = Default::default();
        if let Some(trans) = properties.get("Lcl Translation") {
            let values = trans.get_vec_f32().unwrap();
            translation[0] = values[0];
            translation[1] = values[1];
            translation[2] = values[2];
        }

        // Same for rotation
        let mut rotation: [f32; 3] = Default::default();
        if let Some(rot) = properties.get("Lcl Rotation") {
            let values = rot.get_vec_f32().unwrap();
            rotation[0] = values[0];
            rotation[1] = values[1];
            rotation[2] = values[2];
        }

        // Same for scale
        let mut scale: [f32; 3] = [1.0, 1.0, 1.0];
        if let Some(sca) = properties.get("Lcl Scaling") {
            let values = sca.get_vec_f32().unwrap();
            scale[0] = values[0];
            scale[1] = values[1];
            scale[2] = values[2];
        }

        // Same for pivot
        let mut rotation_pivot: [f32; 3] = Default::default();
        if let Some(piv) = properties.get("RotationPivot") {
            let values = piv.get_vec_f32().unwrap();
            rotation_pivot[0] = values[0];
            rotation_pivot[1] = values[1];
            rotation_pivot[2] = values[2];
        }

        // Retrieve model parameter information
        let model = Model {
            id: node.properties[0].get_i64().unwrap(),
            name: node.properties[1].get_string().unwrap().clone(),
            translation: translation,
            rotation: rotation,
            scale: scale,
            rotation_pivot: rotation_pivot,
        };

        model
    }
}

fn read_properties(node: &RawNode) -> HashMap<String, OwnedProperty> {
    let mut properties = HashMap::new();
    for property_node in &node.children {
        // Get the property's name and flags
        let name = property_node.properties[0].get_string().unwrap().clone();
        //let flags = property_node.properties[3].clone();

        // Read in the rest of the values
        let mut vec = Vec::new();
        for p in &property_node.properties[4..] {
            // TODO: Support anything other than a f32 array
            if let Some(v) = p.get_f32() {
                vec.push(v);
            }
        }
        let value = OwnedProperty::VecF32(vec);

        properties.insert(name, value);
    }
    properties
}

use {RawNode};

#[derive(Clone, Debug, Default)]
pub struct AnimationCurve {
    pub frames: i32,
    pub values: Vec<f32>,
}

impl AnimationCurve {
    pub fn from_node(node: &RawNode) -> Self {
        // Just parse in the entire list of key values, we're assuming each is a frame which will
        // be correct for baked animations. TODO: Support non-baked
        let values = node_to_floats(node.find_child("KeyValueFloat").unwrap());

        AnimationCurve {
            frames: values.len() as i32,
            values: values,
        }
    }
}

fn node_to_floats(node: &RawNode) -> Vec<f32> {
    node.properties[0].get_vec_f32().unwrap().iter().map(|v| *v).collect()
}

use {RawNode, OwnedProperty};

#[derive(Clone, Debug, Default)]
pub struct AnimationCurve {
    pub frames: i32,
    pub values: Vec<OwnedProperty>,
}

impl AnimationCurve {
    pub fn from_node(_node: &RawNode) -> Self {
        AnimationCurve {
            frames: 0,
            values: Vec::new(),
        }
    }
}

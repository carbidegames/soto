use {RawNode};

#[derive(Clone, Debug, Default)]
pub struct AnimationCurve {
    pub frames: i32,
}

impl AnimationCurve {
    pub fn from_node(_node: &RawNode) -> Self {
        AnimationCurve {
            frames: 0,
        }
    }
}

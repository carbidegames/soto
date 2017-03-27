use simple::{SimpleFbx, ObjectId, ObjectType};

pub struct Animation {
    curve_nodes: Vec<ObjectId>,
}

impl Animation {
    pub fn from_simple(fbx: &SimpleFbx) -> Option<Self> {
        // Find the first animation stack we can see TODO: Support multiple
        if let Some((&stack_id, ref _stack)) = fbx.objects.iter()
            .find(|&(_, o)| o.class.type_name() == "AnimationStack") {
            // Found a stack to attach to, now we need all its curve nodes
            let mut curve_nodes = Vec::new();
            for layer in fbx.children_of(stack_id) {
                for node in fbx.children_of(layer.id) {
                    curve_nodes.push(node.id);
                }
            }

            Some(Animation {
                curve_nodes: curve_nodes,
            })
        } else {
            // Nothing found
            None
        }
    }

    pub fn frame_count(&self, fbx: &SimpleFbx) -> i32 {
        // Find the first curve node with curves attached
        for &node in &self.curve_nodes {
            let curves = fbx.driven_properties_of(node);
            if curves.len() > 0 {
                let curve_obj = &fbx.objects[&curves[0].driver];

                // We've found a curve, return its frame count
                if let ObjectType::AnimationCurve(ref curve) = curve_obj.class {
                    return curve.frames
                } else {
                    panic!("Non-curve when curve was expected");
                }
            }
        }

        // We didn't find anything, just return 0
        0
    }

    pub fn transform_fbx_to_frame(&self, _fbx: &mut SimpleFbx, _frame: i32) {
    }
}

#[cfg(test)]
mod tests {
    use simple::{ObjectType, AnimationCurve};
    use super::*;

    #[test]
    fn it_doesnt_create_an_animation_for_animationless_fbx() {
        let anim = Animation::from_simple(&SimpleFbx::new());
        assert!(anim.is_none());
    }

    #[test]
    fn it_gives_zero_frames_for_empty_stack() {
        let mut fbx = SimpleFbx::new();
        fbx.new_object(ObjectType::AnimationStack);

        let anim = Animation::from_simple(&fbx).unwrap();

        assert!(anim.frame_count(&fbx) == 0);
    }

    #[test]
    fn it_gives_frames_for_stack_layer_node_and_curve() {
        let fbx = init_fbx_with_node();

        let anim = Animation::from_simple(&fbx).unwrap();

        assert!(anim.frame_count(&fbx) == 2);
    }

    #[test]
    fn it_transforms_property() {
        let mut fbx = init_fbx_with_node();
        let anim = Animation::from_simple(&fbx).unwrap();

        anim.transform_fbx_to_frame(&mut fbx, 2);

        let model = fbx.objects.iter().find(|&(_, o)| o.class.type_name() == "Model").unwrap();
        assert!(model.1.properties["Blah"].values[0].get_i32().unwrap() == 2);
    }

    fn init_fbx_with_node() -> SimpleFbx {
        let mut fbx = SimpleFbx::new();

        let stack_id = fbx.new_object(ObjectType::AnimationStack);

        let layer_id = fbx.new_object(ObjectType::AnimationLayer);
        fbx.connect_parent_child(stack_id, layer_id);

        let node_id = fbx.new_object(ObjectType::AnimationCurveNode);
        fbx.connect_parent_child(layer_id, node_id);

        let curve_id = fbx.new_object(ObjectType::AnimationCurve(AnimationCurve {
            frames: 2,
        }));
        fbx.connect_property_object(node_id, "d|Blah", curve_id);

        fbx
    }
}

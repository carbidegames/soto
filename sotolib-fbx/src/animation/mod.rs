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
                    panic!("Non-curve when curve was expected, found: {:?}", curve_obj);
                }
            }
        }

        // We didn't find anything, just return 0
        0
    }

    pub fn transform_fbx_to_frame(&self, fbx: &mut SimpleFbx, frame: i32) {
        // Go over all curve nodes of this animation
        for &node in &self.curve_nodes {
            // First, update the node using curves that drive it
            for prop in fbx.driven_properties_of(node) {
                let frame_value = {
                    // Get the curve itself
                    let curve = &fbx.objects[&prop.driver].class.as_animation_curve().unwrap();

                    // Now, apply the curve on the node's property
                    curve.values[frame as usize].clone()
                };

                // Get the property we need to change
                let affected_prop = fbx.objects.get_mut(&node).unwrap()
                    .properties.get_mut(&prop.name).unwrap();

                // Apply the change
                affected_prop.values[0] = frame_value;
            }

            // Find the properties this node is affecting
            for driven_prop in fbx.driving_properties_of(node) {
                // Get this node's values
                let values: Vec<_> = fbx.objects[&node].properties.iter()
                    .map(|(_key, value)| value.values[0].clone())
                    .enumerate().collect();

                // Get the property we need to change
                let affected_prop = fbx.objects.get_mut(&driven_prop.driven).unwrap()
                    .properties.get_mut(&driven_prop.name).unwrap();

                // Apply the node's values to it
                for (i, val) in values {
                    affected_prop.values[i] = val;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use simple::{ObjectType, AnimationCurve, Property};
    use OwnedProperty;
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

        anim.transform_fbx_to_frame(&mut fbx, 1);

        let model = fbx.objects.iter().find(|&(_, o)| o.class.type_name() == "FooBar").unwrap();
        assert!(model.1.properties["Blah"].values[0].get_i32().unwrap() == 2);
    }

    fn init_fbx_with_node() -> SimpleFbx {
        let mut fbx = SimpleFbx::new();

        let foobar_id = fbx.new_object(ObjectType::Other("FooBar".into()));
        {
            let foobar = fbx.objects.get_mut(&foobar_id).unwrap();
            foobar.properties.insert("Blah".into(), Property {
                name: "Blah".into(),
                values: vec!(OwnedProperty::I32(0)),
            });
        }
        fbx.connect_parent_child(0, foobar_id);

        let stack_id = fbx.new_object(ObjectType::AnimationStack);

        let layer_id = fbx.new_object(ObjectType::AnimationLayer);
        fbx.connect_parent_child(stack_id, layer_id);

        let node_id = fbx.new_object(ObjectType::AnimationCurveNode);
        {
            let node = fbx.objects.get_mut(&node_id).unwrap();
            node.properties.insert("d|Blah".into(), Property {
                name: "d|Blah".into(),
                values: vec!(OwnedProperty::I32(0)),
            });
        }
        fbx.connect_parent_child(layer_id, node_id);
        fbx.connect_property_object(foobar_id, "Blah", node_id);

        let curve_id = fbx.new_object(ObjectType::AnimationCurve(AnimationCurve {
            frames: 2,
            values: vec!(
                OwnedProperty::I32(1),
                OwnedProperty::I32(2),
            )
        }));
        fbx.connect_property_object(node_id, "d|Blah", curve_id);

        fbx
    }
}

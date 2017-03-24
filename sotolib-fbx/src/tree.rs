use simple::{SimpleFbx, Object};

#[derive(Debug)]
pub struct ObjectTreeNode {
    pub nodes: Vec<ObjectTreeNode>,
    pub object: Object,
}

impl ObjectTreeNode {
    pub fn from_simple(fbx: &SimpleFbx) -> Self {
        Self::from_object(fbx, Object::new_root())
    }

    fn from_object(fbx: &SimpleFbx, obj: Object) -> Self {
        let mut nodes = Vec::new();

        // Add all children as well
        for child in fbx.children_of(obj.id) {
            nodes.push(Self::from_object(fbx, child.clone()));
        }

        ObjectTreeNode {
            nodes: nodes,
            object: obj
        }
    }
}

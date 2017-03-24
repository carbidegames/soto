use {SimpleFbx, Object};

#[derive(Debug)]
pub struct FbxObjectTreeNode {
    pub nodes: Vec<FbxObjectTreeNode>,
    pub object: Object,
}

impl FbxObjectTreeNode {
    pub fn from_simple(fbx: &SimpleFbx) -> Self {
        Self::from_object(fbx, Object::new_root())
    }

    fn from_object(fbx: &SimpleFbx, obj: Object) -> Self {
        let mut nodes = Vec::new();

        // Add all children as well
        for child in fbx.children_of(obj.id()) {
            nodes.push(Self::from_object(fbx, child.clone()));
        }

        FbxObjectTreeNode {
            nodes: nodes,
            object: obj
        }
    }
}

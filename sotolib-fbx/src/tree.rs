use {SimpleFbx, FbxObject};

#[derive(Debug)]
pub struct FbxObjectTreeNode {
    pub nodes: Vec<FbxObjectTreeNode>,
    pub object: FbxObject,
}

impl FbxObjectTreeNode {
    pub fn from_simple(fbx: &SimpleFbx) -> Self {
        Self::from_object(fbx, FbxObject::Root)
    }

    fn from_object(fbx: &SimpleFbx, obj: FbxObject) -> Self {
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

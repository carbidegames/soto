use std::collections::HashMap;
use {RawFbx, FbxNode};

#[derive(Debug)]
pub enum FbxObject {
    Geometry(FbxGeometry),
    Model(FbxModel),
    /// Currently unsupported object type.
    NotSupported(String)
}

#[derive(Debug)]
pub struct FbxGeometry {
    id: i64,
    name: String,
}

impl FbxGeometry {
    fn from_node(node: &FbxNode) -> Self {
        // Retrieve model information
        let geom = FbxGeometry {
            id: node.properties[0].get_i64().unwrap(),
            name: node.properties[1].get_string().unwrap().clone()
        };

        geom
    }
}

#[derive(Debug)]
pub struct FbxModel {
    id: i64,
    name: String,
}

impl FbxModel {
    fn from_node(node: &FbxNode) -> Self {
        // Retrieve model information
        let model = FbxModel {
            id: node.properties[0].get_i64().unwrap(),
            name: node.properties[1].get_string().unwrap().clone()
        };

        model
    }
}

/// Represents a connection within the FBX file. Connections are laid out (Child, Parent).
#[derive(Debug)]
pub enum FbxConnection {
    /// Object ID to Object ID connections.
    ObjectObject(i64, i64),
    /// Currently unsupported connection type.
    NotSupported(String),
}

#[derive(Debug)]
pub struct SimpleFbx {
    pub objects: HashMap<i64, FbxObject>,
    pub connections: Vec<FbxConnection>,
}

impl SimpleFbx {
    pub fn from_raw(fbx: &RawFbx) -> Self {
        SimpleFbx {
            objects: get_objects(fbx),
            connections: get_connections(fbx),
        }
    }
}

fn get_objects(fbx: &RawFbx) -> HashMap<i64, FbxObject> {
    // Get the node for objects itself
    let objects = fbx.nodes.iter().find(|n| n.name == "Objects").unwrap();
    let mut objs_map = HashMap::new();

    // Go through all the nodes in there
    for node in &objects.nodes {
        // Send this node over to the appropriate converter
        match node.name.as_str() {
            "Geometry" => {
                let geom = FbxGeometry::from_node(node);
                objs_map.insert(geom.id, FbxObject::Geometry(geom));
            }
            "Model" => {
                let model = FbxModel::from_node(node);
                objs_map.insert(model.id, FbxObject::Model(model));
            },
            _ => {
                objs_map.insert(
                    node.properties[0].get_i64().unwrap(),
                    FbxObject::NotSupported(node.name.clone())
                );
            },
        }
    }

    objs_map
}

fn get_connections(fbx: &RawFbx) -> Vec<FbxConnection> {
    // Get the node for connections itself
    let connections = fbx.nodes.iter().find(|n| n.name == "Connections").unwrap();
    let mut con_vec = Vec::new();

    // Go through all the nodes in there
    for node in &connections.nodes {
        let con = match node.properties[0].get_string().unwrap().as_str() {
            "OO" => FbxConnection::ObjectObject(
                node.properties[1].get_i64().unwrap(),
                node.properties[2].get_i64().unwrap(),
            ),
            other => FbxConnection::NotSupported(other.to_string())
        };
        con_vec.push(con);
    }

    con_vec
}

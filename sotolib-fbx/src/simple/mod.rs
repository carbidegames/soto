mod geometry;
mod model;
mod object;

pub use self::geometry::{Geometry};
pub use self::model::{Model};
pub use self::object::{Object, ObjectType};

use std::collections::HashMap;
use {RawFbx};

/// Represents a connection within the FBX file. Connections are laid out (Child, Parent).
#[derive(Debug)]
pub enum Connection {
    /// Object ID to Object ID connections.
    ObjectObject(i64, i64),
    /// Object ID to Object ID + PropertyKey connections.
    ObjectProperty(i64, i64, String),
    /// Currently unsupported connection type.
    NotSupported(String),
}

#[derive(Debug)]
pub struct SimpleFbx {
    pub objects: HashMap<i64, Object>,
    pub connections: Vec<Connection>,
}

impl SimpleFbx {
    pub fn from_raw(fbx: &RawFbx) -> Self {
        SimpleFbx {
            objects: get_objects(fbx),
            connections: get_connections(fbx),
        }
    }

    /// Gets all objects that are linked as children of another object by the parent's id.
    pub fn children_of(&self, id: i64) -> Vec<&Object> {
        let mut objs = Vec::new();

        // Go through all connections
        for connection in &self.connections {
            if let &Connection::ObjectObject(child, parent) = connection {
                if parent == id {
                    // We've found one, look it up and add it
                    objs.push(&self.objects[&child])
                }
            }
        }

        objs
    }
}

fn get_objects(fbx: &RawFbx) -> HashMap<i64, Object> {
    // Get the node for objects itself
    let objects = fbx.nodes.iter().find(|n| n.name == "Objects").unwrap();
    let mut objs_map = HashMap::new();

    // Go through all the nodes in there
    for node in &objects.children {
        // Send this node over to the appropriate converter
        match node.name.as_str() {
            "Geometry" => {
                let geom = Geometry::from_node(node);
                objs_map.insert(geom.id, Object { class: ObjectType::Geometry(geom) });
            }
            "Model" => {
                let model = Model::from_node(node);
                objs_map.insert(model.id, Object { class: ObjectType::Model(model) });
            },
            _ => {
                objs_map.insert(
                    node.properties[0].get_i64().unwrap(),
                    Object { class: ObjectType::NotSupported(node.name.clone()) }
                );
            },
        }
    }

    objs_map
}

fn get_connections(fbx: &RawFbx) -> Vec<Connection> {
    // Get the node for connections itself
    let connections = fbx.nodes.iter().find(|n| n.name == "Connections").unwrap();
    let mut con_vec = Vec::new();

    // Go through all the nodes in there
    for node in &connections.children {
        let con = match node.properties[0].get_string().unwrap().as_str() {
            "OO" => Connection::ObjectObject(
                node.properties[1].get_i64().unwrap(),
                node.properties[2].get_i64().unwrap(),
            ),
            "OP" => Connection::ObjectProperty(
                node.properties[1].get_i64().unwrap(),
                node.properties[2].get_i64().unwrap(),
                node.properties[3].get_string().unwrap().clone(),
            ),
            other => Connection::NotSupported(other.to_string())
        };
        con_vec.push(con);
    }

    con_vec
}

mod animation;
mod geometry;
mod model;
mod object;
mod properties;

pub use self::animation::{AnimationCurve};
pub use self::geometry::{Geometry};
pub use self::model::{Model, ModelProperties};
pub use self::object::{Object, ObjectType};
pub use self::properties::{Property, Properties};

use std::collections::{HashMap};
use {RawFbx, Error};

pub type ObjectId = i64;

/// Represents a connection within the FBX file. Connections are laid out (Child, Parent).
#[derive(Debug)]
pub enum Connection {
    /// Object ID to Object ID connections.
    ObjectObject(ObjectId, ObjectId),
    /// Object ID to Object ID + PropertyKey connections.
    ObjectProperty(ObjectId, ObjectId, String),
    /// Currently unsupported connection type.
    NotSupported(String),
}

#[derive(Debug, Default)]
pub struct SimpleFbx {
    pub objects: HashMap<ObjectId, Object>,
    pub connections: Vec<Connection>,
}

impl SimpleFbx {
    pub fn new() -> Self {
        SimpleFbx::default()
    }

    pub fn from_raw(fbx: &RawFbx) -> Result<Self, Error> {
        Ok(SimpleFbx {
            objects: get_objects(fbx)?,
            connections: get_connections(fbx),
        })
    }

    pub fn new_object(&mut self, class: ObjectType) -> ObjectId {
        // Find an unused id for this object
        let mut id = 1;
        for (_, obj) in &self.objects {
            if obj.id >= id {
                id = obj.id + 1;
            }
        }

        // Add the actual object
        self.objects.insert(id, Object {
            id: id,
            name: class.type_name(),
            properties: Properties::new(),
            class: class,
        });

        id
    }

    pub fn connect_parent_child(&mut self, parent: ObjectId, child: ObjectId) {
        self.connections.push(Connection::ObjectObject(child, parent));
    }

    pub fn connect_property_object(&mut self, driven: ObjectId, property: &str, driver: ObjectId) {
        self.connections.push(Connection::ObjectProperty(
            driver,
            driven, property.into(),
        ));
    }

    /// Gets all objects that are linked as children of another object by the parent's id.
    pub fn children_of(&self, id: ObjectId) -> Vec<&Object> {
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

    pub fn driven_properties_of(&self, driven: ObjectId) -> Vec<DrivenProperty> {
        let mut vec = Vec::new();

        // Go through all connections
        for connection in &self.connections {
            // Check if this is a Object-Property connection with the driven
            if let Connection::ObjectProperty(driver, c_driven, ref property) = *connection {
                if c_driven != driven {
                    continue;
                }

                // We've got one, add it to the list
                vec.push(DrivenProperty {
                    name: property.clone(),
                    driver: driver,
                    driven: driven,
                });
            }
        }

        vec
    }

    pub fn driving_properties_of(&self, driver: ObjectId) -> Vec<DrivenProperty> {
        let mut vec = Vec::new();

        // Go through all connections
        for connection in &self.connections {
            // Check if this is a Object-Property connection with the driver
            if let Connection::ObjectProperty(c_driver, driven, ref property) = *connection {
                if c_driver != driver {
                    continue;
                }

                // We've got one, add it to the list
                vec.push(DrivenProperty {
                    name: property.clone(),
                    driver: driver,
                    driven: driven,
                });
            }
        }

        vec
    }
}

pub struct DrivenProperty {
    pub name: String,
    pub driver: ObjectId,
    pub driven: ObjectId,
}

fn get_objects(fbx: &RawFbx) -> Result<HashMap<i64, Object>, Error> {
    // Get the node for objects itself
    let objects = fbx.nodes.iter().find(|n| n.name == "Objects").unwrap();
    let mut objs_map = HashMap::new();

    // Go through all the nodes in there and add them
    for node in &objects.children {
        let obj = Object::from_node(&node)?;
        objs_map.insert(obj.id, obj);
    }

    Ok(objs_map)
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_connects_parents_to_children() {
        let mut fbx = SimpleFbx::new();
        let parent = fbx.new_object(ObjectType::Model(Default::default()));
        let child = fbx.new_object(ObjectType::Geometry(Default::default()));

        fbx.connect_parent_child(parent, child);

        let children = fbx.children_of(parent);
        assert!(children.len() == 1);
        assert!(children[0].id == child);
    }

    #[test]
    fn it_connects_properties_to_objects() {
        let mut fbx = SimpleFbx::new();
        let driven = fbx.new_object(ObjectType::Model(Default::default()));
        let driver = fbx.new_object(ObjectType::Geometry(Default::default()));

        fbx.connect_property_object(driven, "d|Blah", driver);

        let props = fbx.driven_properties_of(driven);
        assert!(props.len() == 1);
        assert!(props[0].name == "d|Blah");
        assert!(props[0].driver == driver);
    }
}

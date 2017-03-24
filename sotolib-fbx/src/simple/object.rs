use simple::{Geometry, Model, Properties};
use {RawNode};

#[derive(Debug, Clone)]
pub struct Object {
    pub id: i64,
    pub name: String,
    properties: Properties,
    /// Contains the type and type-specific data.
    pub class: ObjectType,
}

impl Object {
    pub fn new_root() -> Self {
        Object {
            id: 0,
            name: "Root".into(),
            properties: Properties::default(),
            class: ObjectType::Root,
        }
    }

    pub fn from_node(node: &RawNode) -> Self {
        // Generic data
        let id = node.properties[0].get_i64().unwrap();
        let name = node.properties[1].get_string().unwrap().clone();

        // Specific object type
        let class = match node.name.as_str() {
            "Geometry" => {
                ObjectType::Geometry(Geometry::from_node(node))
            }
            "Model" => {
                ObjectType::Model(Model::from_node(node))
            },
            _ => {
                ObjectType::NotSupported(node.name.clone())
            },
        };

        Object {
            id: id,
            class: class,
            properties: Properties::from_node(node.find_child("Properties70").unwrap()),
            name: name,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ObjectType {
    Geometry(Geometry),
    Model(Model),
    /// Virtual object representing the root of the file.
    Root,
    /// Currently unsupported object type.
    NotSupported(String)
}

#[cfg(test)]
mod tests {
    use fbx_direct::common::OwnedProperty;
    use RawNode;
    use super::*;

    #[test]
    fn it_parses_id_and_name() {
        let expected_id = 123454321;
        let expected_name = "Smorgasbord of Alots";

        let obj = Object::from_node(&RawNode {
            name: "Smorgasbord".into(),
            properties: vec!(
                OwnedProperty::I64(expected_id),
                OwnedProperty::String(expected_name.into())
            ),
            children: Vec::new(),
        });

        assert!(obj.id == expected_id);
        assert!(obj.name == expected_name);
    }
}

use simple::{Geometry, Model, Property, Properties, AnimationCurve};
use {RawNode, Error};

#[derive(Debug, Clone)]
pub struct Object {
    pub id: i64,
    pub name: String,
    pub properties: Properties,
    /// Contains the type and type-specific data.
    pub class: ObjectType,
}

impl Object {
    pub fn new_root() -> Self {
        Object {
            id: 0,
            name: "Root".into(),
            properties: Properties::new(),
            class: ObjectType::Root,
        }
    }

    pub fn from_node(node: &RawNode) -> Result<Self, Error> {
        // Generic data
        let id = node.properties[0].get_i64().unwrap();
        let name = node.properties[1].get_string().unwrap().clone();

        // Properties, of which there may be none
        let properties = if let Some(props_node) = node.find_child("Properties70") {
            let p: Result<Vec<_>, _> = props_node.children.iter()
                .map(|c| Property::from_node(c))
                .collect();
            p?.into_iter()
                .map(|p| (p.name.clone(), p))
                .collect()
        } else {
            Properties::new()
        };

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

        Ok(Object {
            id: id,
            class: class,
            properties: properties,
            name: name,
        })
    }
}

#[derive(Debug, Clone)]
pub enum ObjectType {
    AnimationStack,
    AnimationLayer,
    AnimationCurveNode,
    AnimationCurve(AnimationCurve),
    Geometry(Geometry),
    Model(Model),
    /// Virtual object representing the root of the file.
    Root,
    /// Currently unsupported object type.
    NotSupported(String)
}

impl ObjectType {
    pub fn type_name(&self) -> String {
        match *self {
            ObjectType::AnimationStack => "AnimationStack",
            ObjectType::AnimationLayer => "AnimationLayer",
            ObjectType::AnimationCurveNode => "AnimationCurveNode",
            ObjectType::AnimationCurve(_) => "AnimationCurve",
            ObjectType::Geometry(_) => "Geometry",
            ObjectType::Model(_) => "Model",
            ObjectType::Root => "Root", // This really should never be used but here we go
            ObjectType::NotSupported(ref t) => &t,
        }.into()
    }
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
        }).unwrap();

        assert!(obj.id == expected_id);
        assert!(obj.name == expected_name);
    }
}

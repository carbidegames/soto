use std::collections::HashMap;
use {RawNode, OwnedProperty, Error};

#[derive(Default, Debug, Clone)]
pub struct Property {
    pub name: String,
    pub values: Vec<OwnedProperty>
}

impl Property {
    pub fn from_node(node: &RawNode) -> Result<Self, Error> {
        // Validation
        if node.name != "P" {
            return Err(Error::WrongNode("P".into(), node.name.clone()))
        }
        if node.properties.len() < 3 {
            return Err(
                Error::WrongNodeLayout("Minimum of 3 properties required in Property node".into())
            )
        }

        Ok(Property {
            name: node.properties[0].get_string().unwrap().clone(),
            values: node.properties[4..].iter().map(|v| v.clone()).collect(),
        })
    }

    pub fn to_vector3(&self) -> [f32; 3] {
        [self.values[0].get_f32().unwrap(),
        self.values[1].get_f32().unwrap(),
        self.values[2].get_f32().unwrap()]
    }
}

pub type Properties = HashMap<String, Property>;

#[cfg(test)]
mod tests {
    use {OwnedProperty, RawNode, Error};
    use super::*;

    #[test]
    fn it_parses_properties() {
        let expected_name = "d|X";
        let expected_value = OwnedProperty::F32(1.0);

        let prop = Property::from_node(&RawNode {
            name: "P".into(),
            properties: vec!(
                OwnedProperty::String(expected_name.into()),
                OwnedProperty::String("Number".into()),
                OwnedProperty::String("".into()),
                OwnedProperty::String("A".into()),
                expected_value.clone(),
            ),
            children: Vec::new(),
        }).unwrap();

        assert!(prop.name == expected_name);
        assert!(prop.values.len() == 1);
        assert!(prop.values[0] == expected_value);
    }

    #[test]
    fn it_refuses_invalid_data() {
        let invalid_name_property = Property::from_node(&RawNode {
            name: "NotAProperty".into(),
            properties: vec!(
                OwnedProperty::String("Blah".into()),
                OwnedProperty::String("Number".into()),
                OwnedProperty::String("".into()),
                OwnedProperty::String("A".into()),
                OwnedProperty::String("Blah".into()),
            ),
            children: Vec::new(),
        });
        let invalid_length_property = Property::from_node(&RawNode {
            name: "P".into(),
            properties: vec!(
                OwnedProperty::String("Blah".into()),
            ),
            children: Vec::new(),
        });

        if let Err(Error::WrongNode(_, _)) = invalid_name_property {
        } else {
            assert!(invalid_name_property.is_err(), "Didn't receive an error");
            assert!(false, "Didn't receive right error");
        }

        if let Err(Error::WrongNodeLayout(_)) = invalid_length_property {
        } else {
            assert!(invalid_length_property.is_err(), "Didn't receive an error");
            assert!(false, "Didn't receive right error");
        }
    }
}

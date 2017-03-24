use {RawNode, OwnedProperty};

#[derive(Default, Debug, Clone)]
pub struct Property {
    name: String,
    values: Vec<OwnedProperty>
}

impl Property {
    pub fn from_node(node: &RawNode) -> Self {
        Property {
            name: node.properties[0].get_string().unwrap().clone(),
            values: node.properties[4..].iter().map(|v| v.clone()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use {OwnedProperty, RawNode};
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
        });

        assert!(prop.name == expected_name);
        assert!(prop.values.len() == 1);
        assert!(prop.values[0] == expected_value);
    }
}

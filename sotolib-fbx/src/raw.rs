use std::io::{Read};

use fbx_direct::common::OwnedProperty;
use fbx_direct::reader::{FbxEvent, EventReader};

use {FbxDirectError, Error};

#[derive(Debug)]
pub struct RawNode {
    pub name: String,
    pub properties: Vec<OwnedProperty>,
    pub children: Vec<RawNode>,
}

impl RawNode {
    fn parse<R: Read>(name: String, properties: Vec<OwnedProperty>, parser: &mut EventReader<R>) -> Result<Self, FbxDirectError> {
        let mut node = RawNode {
            name: name,
            properties: properties,
            children: Vec::new(),
        };

        loop {
            match parser.next()? {
                FbxEvent::StartNode { name, properties } => {
                    let new_node = RawNode::parse(name, properties, parser)?;
                    node.children.push(new_node);
                },
                FbxEvent::EndNode => break,
                _ => {}
            }
        }

        Ok(node)
    }

    pub fn find_child(&self, name: &str) -> Option<&RawNode> {
        self.children.iter().find(|c| c.name == name)
    }
}

#[derive(Debug)]
pub struct RawFbx {
    pub nodes: Vec<RawNode>,
}

impl RawFbx {
    pub fn parse<R: Read>(read: R) -> Result<Self, Error> {
        // Set up the parser
        let mut parser = EventReader::new(read);
        let mut fbx = RawFbx {
            nodes: Vec::new(),
        };

        // Go over all events
        loop {
            // Check what event we got
            match parser.next().map_err(|e| Error::FbxDirect(e))? {
                // If we get to the start of a node
                FbxEvent::StartNode { name, properties } => {
                    let new_node = RawNode::parse(name, properties, &mut parser)
                        .map_err(|e| Error::FbxDirect(e))?;
                    fbx.nodes.push(new_node);
                },
                // If we hit the end of the file, we're done
                FbxEvent::EndFbx => break,
                _ => {}
            }
        }

        Ok(fbx)
    }
}

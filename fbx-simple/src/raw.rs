use std::io::{Read};

use fbx_direct::common::OwnedProperty;
use fbx_direct::reader::{FbxEvent, EventReader};

use {FbxDirectError, FbxError};

#[derive(Debug)]
pub struct FbxNode {
    pub name: String,
    pub properties: Vec<OwnedProperty>,
    pub nodes: Vec<FbxNode>,
}

impl FbxNode {
    fn parse<R: Read>(name: String, properties: Vec<OwnedProperty>, parser: &mut EventReader<R>) -> Result<Self, FbxDirectError> {
        let mut node = FbxNode {
            name: name,
            properties: properties,
            nodes: Vec::new(),
        };

        loop {
            match parser.next()? {
                FbxEvent::StartNode { name, properties } => {
                    let new_node = FbxNode::parse(name, properties, parser)?;
                    node.nodes.push(new_node);
                },
                FbxEvent::EndNode => break,
                _ => {}
            }
        }

        Ok(node)
    }
}

#[derive(Debug)]
pub struct RawFbx {
    pub nodes: Vec<FbxNode>,
}

impl RawFbx {
    pub fn parse<R: Read>(read: R) -> Result<Self, FbxError> {
        // Set up the parser
        let mut parser = EventReader::new(read);
        let mut fbx = RawFbx {
            nodes: Vec::new(),
        };

        // Go over all events
        loop {
            // Check what event we got
            match parser.next().map_err(|e| FbxError::FbxDirect(e))? {
                // If we get to the start of a node
                FbxEvent::StartNode { name, properties } => {
                    let new_node = FbxNode::parse(name, properties, &mut parser)
                        .map_err(|e| FbxError::FbxDirect(e))?;
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

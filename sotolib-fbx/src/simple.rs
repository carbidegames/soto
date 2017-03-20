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
    pub id: i64,
    pub name: String,
    /// Vertices that make up the polygon.
    pub vertices: Vec<[f32; 3]>,
    /// Vertex indices that make up the polygons.
    pub polygons: Vec<Vec<u32>>,
    /// Normals for polygon vertices. Other normal types than "ByPolygonVertex", "Direct" not
    /// currently supported.
    pub normals: Vec<[f32; 3]>,
    /// UVs for polygon vertices. Other uv types than "ByPolygonVertex", "IndexToDirect" not
    /// currently supported.
    pub uvs: Vec<[f32; 2]>,
}

impl FbxGeometry {
    fn from_node(node: &FbxNode) -> Self {
        // Read in the vertex data, which is just stored in the sub-node "Vertices"
        let vert_node = node.find_child("Vertices").unwrap();
        let vertices = node_to_vector3s(vert_node);

        // Read in the indices
        let indi_node = node.find_child("PolygonVertexIndex").unwrap();
        let mut polygons = Vec::new();
        let mut cur_polygon = Vec::new();
        for val in indi_node.properties[0].get_vec_i32().unwrap().iter() {
            let mut val: i32 = *val;

            // If this one is negative we need to adjust it and that means this is end of the polygon
            let end = if val < 0 {
                val = -val-1;
                true
            } else { false };

            // Add it to the polygon
            cur_polygon.push(val as u32);

            // If this is the end, add this polygon to the list and create a new one
            if end {
                polygons.push(cur_polygon);
                cur_polygon = Vec::new();
            }
        }
        assert!(cur_polygon.len() == 0);

        // Read in the normals (only supports Direct)
        let norm_node = node.find_child("LayerElementNormal").unwrap();
        assert!(
            norm_node.find_child("MappingInformationType").unwrap()
                .properties[0].get_string().unwrap() == "ByPolygonVertex");
        assert!(
            norm_node.find_child("ReferenceInformationType").unwrap()
                .properties[0].get_string().unwrap() == "Direct");
        let norm_values_node = norm_node.find_child("Normals").unwrap();
        let normals = node_to_vector3s(norm_values_node);

        // Read in the uvs (only supports IndexToDirect)
        let uvs_node = node.find_child("LayerElementUV").unwrap();
        assert!(
            uvs_node.find_child("MappingInformationType").unwrap()
                .properties[0].get_string().unwrap() == "ByPolygonVertex");
        assert!(
            uvs_node.find_child("ReferenceInformationType").unwrap()
                .properties[0].get_string().unwrap() == "IndexToDirect");
        let uvs_raw = node_to_vector2s(uvs_node.find_child("UV").unwrap());
        let uvs_indices = uvs_node.find_child("UVIndex").unwrap().properties[0].get_vec_i32().unwrap();
        let mut uvs = Vec::new();
        for index in uvs_indices.iter() {
            uvs.push(uvs_raw[*index as usize]);
        }

        // Finish off the geometry type
        FbxGeometry {
            id: node.properties[0].get_i64().unwrap(),
            name: node.properties[1].get_string().unwrap().clone(),
            vertices: vertices,
            polygons: polygons,
            normals: normals,
            uvs: uvs,
        }
    }

    pub fn triangles(&self) -> Vec<[([f32; 3], [f32; 3], [f32; 2]); 3]> {
        // If our assumptions are right, normals and uvs should have the same amount of entries
        assert!(self.normals.len() == self.uvs.len());

        let mut triangles = Vec::new();

        // Go through all polygons
        for (poly_num, poly) in self.polygons.iter().enumerate() {
            // Make sure this is a triangle, our current method of getting the normals and uvs
            // doesn't make sense otherwise.
            assert!(poly.len() == 3); // TODO: Improve error handling

            // Go through the indices in this polygon
            let mut triangle: [([f32; 3], [f32; 3], [f32; 2]); 3] = Default::default();
            for (index_num, index) in poly.iter().enumerate() {
                triangle[index_num] = (
                    self.vertices[*index as usize],
                    self.normals[poly_num * 3 + index_num],
                    self.uvs[poly_num * 3 + index_num],
                );
            }
            triangles.push(triangle);
        }

        triangles
    }
}

fn node_to_vector3s(node: &FbxNode) -> Vec<[f32; 3]> {
    let mut vectors = Vec::new();

    for val in node.properties[0].get_vec_f32().unwrap().chunks(3) {
        assert!(val.len() == 3);
        vectors.push([val[0], val[1], val[2]]);
    }

    vectors
}

fn node_to_vector2s(node: &FbxNode) -> Vec<[f32; 2]> {
    let mut vectors = Vec::new();

    for val in node.properties[0].get_vec_f32().unwrap().chunks(2) {
        assert!(val.len() == 2);
        vectors.push([val[0], val[1]]);
    }

    vectors
}

#[derive(Debug)]
pub struct FbxModel {
    pub id: i64,
    pub name: String,
    pub translation: [f32; 3],
}

impl FbxModel {
    fn from_node(node: &FbxNode) -> Self {
        // Find the translation in the model
        let translation: [f32; 3] = Default::default();

        // Retrieve model parameter information
        let model = FbxModel {
            id: node.properties[0].get_i64().unwrap(),
            name: node.properties[1].get_string().unwrap().clone(),
            translation: translation,
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

    /// Gets all objects that are linked as children of another object by the parent's id.
    pub fn children_of(&self, id: i64) -> Vec<&FbxObject> {
        let mut objs = Vec::new();

        // Go through all connections
        for connection in &self.connections {
            if let &FbxConnection::ObjectObject(child, parent) = connection {
                if parent == id {
                    // We've found one, look it up and add it
                    objs.push(&self.objects[&child])
                }
            }
        }

        objs
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

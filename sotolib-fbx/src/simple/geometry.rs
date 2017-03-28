use {RawNode};

#[derive(Clone, Debug, Default)]
pub struct Geometry {
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

impl Geometry {
    pub fn from_node(node: &RawNode) -> Self {
        // First, make sure we've got a mesh
        // TODO: Support other geometry types
        let class = node.properties[2].get_string().unwrap();
        if class != "Mesh" {
            // It's not a mesh, just return an empty geometry
            return Default::default();
        }

        // Read in the vertex data, which is just stored in the sub-node "Vertices"
        let vert_node = node.find_child("Vertices").unwrap();
        let vertices = node_to_vector3s(vert_node);

        // Read in the indices
        let indi_node = node.find_child("PolygonVertexIndex").unwrap();
        let vertex_indices_raw: Vec<i32> = indi_node.properties[0].clone().into_vec_i32().unwrap();
        let mut polygons = Vec::new();
        let mut cur_polygon = Vec::new();
        let mut vertex_indices = Vec::new();
        for val in &vertex_indices_raw {
            let mut val: i32 = *val;

            // If this one is negative we need to adjust it and that means this is end of the polygon
            let end = if val < 0 {
                val = -val-1;
                true
            } else { false };

            // Now that we have a correct value, keep track of it for later
            vertex_indices.push(val);

            // Add it to the polygon
            cur_polygon.push(val as u32);

            // If this is the end, add this polygon to the list and create a new one
            if end {
                polygons.push(cur_polygon);
                cur_polygon = Vec::new();
            }
        }
        assert!(cur_polygon.len() == 0);

        // Read in the normals
        let normals_node = node.find_child("LayerElementNormal").unwrap();
        let normals_data_node = normals_node.find_child("Normals").unwrap();
        let normals_data = node_to_vector3s(normals_data_node);
        let normals = flatten_mapping_to_vertices(normals_node, normals_data, "NormalsIndex", &vertex_indices);

        // Read in the uvs (only supports IndexToDirect)
        let uvs_node = node.find_child("LayerElementUV").unwrap();
        let uvs_data_node = uvs_node.find_child("UV").unwrap();
        let uvs_data = node_to_vector2s(uvs_data_node);
        let uvs = flatten_mapping_to_vertices(uvs_node, uvs_data, "UVIndex", &vertex_indices);

        // Finish off the geometry type
        Geometry {
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

fn flatten_mapping_to_vertices<T: Copy>(
    node: &RawNode, data_raw: Vec<T>, indices_field_name: &str, vertex_indices: &Vec<i32>
) -> Vec<T> {
    // First flatten the ReferenceInformationType, which can be indices instead of flat values
    let information = node.find_child("ReferenceInformationType").unwrap()
        .properties[0].get_string().unwrap();
    let data_actual = match information.as_str() {
        // It's already mapped correctly
        "Direct" => data_raw,
        // We need to get the UVs and map the data over those
        "IndexToDirect" => {
            let uvs_indices = node.find_child(indices_field_name).unwrap()
                .properties[0].get_vec_i32().unwrap();

            let mut data_actual = Vec::new();
            for index in uvs_indices.iter() {
                data_actual.push(data_raw[*index as usize]);
            }
            data_actual
        }
        // We don't know this type of mapping yet
        // TODO: Improve error handling
        other => panic!("Unknown information type {}", other),
    };

    // Map them according to how we're told to
    let mapping = node.find_child("MappingInformationType").unwrap()
        .properties[0].get_string().unwrap();
    match mapping.as_str() {
        // It's already mapped correctly
        "ByPolygonVertex" => data_actual,
        // This means we need to look at the vertex indices and map our data the same way
        "ByVertice" => {
            let mut data = Vec::new();
            for index in vertex_indices {
                data.push(data_actual[*index as usize]);
            }
            data
        }
        // We don't know this type of mapping yet
        // TODO: Improve error handling
        other => panic!("Unknown mapping type {}", other),
    }
}

fn node_to_vector3s(node: &RawNode) -> Vec<[f32; 3]> {
    let mut vectors = Vec::new();

    for val in node.properties[0].get_vec_f32().unwrap().chunks(3) {
        assert!(val.len() == 3);
        vectors.push([val[0], val[1], val[2]]);
    }

    vectors
}

fn node_to_vector2s(node: &RawNode) -> Vec<[f32; 2]> {
    let mut vectors = Vec::new();

    for val in node.properties[0].get_vec_f32().unwrap().chunks(2) {
        assert!(val.len() == 2);
        vectors.push([val[0], val[1]]);
    }

    vectors
}

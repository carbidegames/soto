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

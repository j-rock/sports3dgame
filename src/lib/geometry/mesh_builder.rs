use geometry::{
    mesh::VertexIndex,
    Mesh
};
use glm;
use std::{
    f32,
    collections::{
        BTreeMap,
        HashMap,
        HashSet
    }
};

#[derive(PartialEq, Eq, Hash)]
struct MeshBuilderVertex(u32, u32, u32);

impl MeshBuilderVertex {
    pub fn from_vec(vec3: glm::Vec3) -> MeshBuilderVertex {
        MeshBuilderVertex(vec3.x.to_bits(), vec3.y.to_bits(), vec3.z.to_bits())
    }

    pub fn into_vec(self) -> glm::Vec3 {
       glm::vec3(f32::from_bits(self.0), f32::from_bits(self.1), f32::from_bits(self.2))
    }
}

struct OriginalFace(VertexIndex, VertexIndex, VertexIndex, VertexIndex);

#[derive(PartialEq, Eq, Hash)]
struct SortedFace(VertexIndex, VertexIndex, VertexIndex, VertexIndex);

impl SortedFace {
    pub fn new(v0: VertexIndex, v1: VertexIndex, v2: VertexIndex, v3: VertexIndex) -> SortedFace {
        let mut vs = vec!(v0, v1, v2, v3);
        vs.sort_unstable();
        SortedFace(vs[0], vs[1], vs[2], vs[3])
    }
}

// Constructs a Mesh from a series of quad-vertex faces.
pub struct QuadMeshBuilder {
    vertex_indices: HashMap<MeshBuilderVertex, VertexIndex>,
    face_bag: HashSet<SortedFace>,
    final_faces: Vec<OriginalFace>,
}

impl QuadMeshBuilder {
    pub fn new() -> QuadMeshBuilder {
        QuadMeshBuilder {
            vertex_indices: HashMap::new(),
            face_bag: HashSet::new(),
            final_faces: vec!(),
        }
    }

    fn add_vertex(&mut self, vertex: MeshBuilderVertex) -> VertexIndex {
        let size_before = self.vertex_indices.len() as VertexIndex;
        let value = self.vertex_indices.entry(vertex).or_insert(size_before);
        *value
    }

    pub fn add_face(&mut self,
                    top_left: glm::Vec3, top_right: glm::Vec3,
                    bottom_left: glm::Vec3, bottom_right: glm::Vec3) {
        let tl_idx= self.add_vertex(MeshBuilderVertex::from_vec(top_left));
        let tr_idx= self.add_vertex(MeshBuilderVertex::from_vec(top_right));
        let bl_idx= self.add_vertex(MeshBuilderVertex::from_vec(bottom_left));
        let br_idx= self.add_vertex(MeshBuilderVertex::from_vec(bottom_right));

        let sorted_face = SortedFace::new(tl_idx, tr_idx, bl_idx, br_idx);
        if self.face_bag.insert(sorted_face) {
            self.final_faces.push(OriginalFace(tl_idx, tr_idx, bl_idx, br_idx));
        }
    }

    pub fn build(self) -> Mesh {
        let mut reverse_index = BTreeMap::new();
        for (vertex, idx) in self.vertex_indices.into_iter() {
            reverse_index.insert(idx, vertex.into_vec());
        }
        let vertices: Vec<_> = reverse_index.values().cloned().collect();
        let mut faces = Vec::with_capacity(6 * self.final_faces.len());
        for &OriginalFace(v0, v1, v2, v3) in self.final_faces.iter() {
            let mut two_triangles = vec!(v0, v2, v3, v1, v0, v3);
            faces.append(&mut two_triangles);
        }
        Mesh::from_geometry(vertices, faces)
    }
}

struct OriginalTriangle(VertexIndex, VertexIndex, VertexIndex);

#[derive(PartialEq, Eq, Hash)]
struct SortedTriangle(VertexIndex, VertexIndex, VertexIndex);

impl SortedTriangle {
    pub fn new(v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) -> SortedTriangle {
        let mut vs = vec!(v0, v1, v2);
        vs.sort_unstable();
        SortedTriangle(vs[0], vs[1], vs[2])
    }
}

// Constructs a Mesh from a series of triangles.
pub struct TriMeshBuilder {
    vertex_indices: HashMap<MeshBuilderVertex, VertexIndex>,
    face_bag: HashSet<SortedTriangle>,
    final_faces: Vec<OriginalTriangle>,
}

impl TriMeshBuilder {
    pub fn new() -> TriMeshBuilder {
        TriMeshBuilder {
            vertex_indices: HashMap::new(),
            face_bag: HashSet::new(),
            final_faces: vec!(),
        }
    }

    fn add_vertex(&mut self, vertex: MeshBuilderVertex) -> VertexIndex {
        let size_before = self.vertex_indices.len() as VertexIndex;
        let value = self.vertex_indices.entry(vertex).or_insert(size_before);
        *value
    }

    pub fn add_triangle(&mut self, v0: glm::Vec3, v1: glm::Vec3, v2: glm::Vec3) {
        let idx_0 = self.add_vertex(MeshBuilderVertex::from_vec(v0));
        let idx_1 = self.add_vertex(MeshBuilderVertex::from_vec(v1));
        let idx_2 = self.add_vertex(MeshBuilderVertex::from_vec(v2));

        let sorted_face = SortedTriangle::new(idx_0, idx_1, idx_2);
        if self.face_bag.insert(sorted_face) {
            self.final_faces.push(OriginalTriangle(idx_0, idx_1, idx_2));
        }
    }

    pub fn build(self) -> Mesh {
        let mut reverse_index = BTreeMap::new();
        for (vertex, idx) in self.vertex_indices.into_iter() {
            reverse_index.insert(idx, vertex.into_vec());
        }
        let vertices: Vec<_> = reverse_index.values().cloned().collect();
        let mut faces = Vec::with_capacity(3 * self.final_faces.len());
        for &OriginalTriangle(v0, v1, v2) in self.final_faces.iter() {
            let mut triangle = vec!(v0, v1, v2);
            faces.append(&mut triangle);
        }
        Mesh::from_geometry(vertices, faces)
    }
}

use bevy::prelude::*;
use crate::core::constants::*;

struct MeshData {
    verticles: Vec<Vec3>,
    normals: Vec<Vec3>,
    uvs: Vec<Vec2>,
    indices: Vec<u32>,
}

impl MeshData {
    fn new() -> Self {
        Self {
            verticles: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn add_vertex(&mut self, position: Vec3, normal: Vec3, uv: Vec2) -> u32 {
        let idx = self.verticles.len() as u32;
        self.verticles.push(position);
        self.normals.push(normal);
        self.uvs.push(uv);
        idx
    }

    fn to_mesh(self) -> Mesh {
        let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList, default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.verticles);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        mesh.insert_indices(bevy::render::mesh::Indices::U32(self.indices));
        mesh
    }
}

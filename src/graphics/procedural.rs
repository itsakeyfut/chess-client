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

fn add_cylinder_to_mesh(mesh_data: &mut MeshData, center: Vec3, radius: f32, height: f32, segments: u32) {
    let start_idx = mesh_data.verticles.len() as u32;

    let top_center = mesh_data.add_vertex(
        center + Vec3::new(0.0, height / 2.0, 0.0),
        Vec3::Y,
        Vec2::new(0.5, 0.5)
    );
    let bottom_center = mesh_data.add_vertex(
        center + Vec3::new(0.0, -height / 2.0, 0.0),
        Vec3::NEG_Y,
        Vec2::new(0.5, 0.5)
    );

    for i in 0..segments {
        let angle = i as f32 * 2.0 * std::f32::consts::PI / segments as f32;
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let x = cos_a * radius;
        let z = sin_a * radius;

        // top vertex
        mesh_data.add_vertex(
            center + Vec3::new(x, height / 2.0, z),
            Vec3::Y,
            Vec2::new((cos_a + 1.0) * 0.5, (sin_a + 1.0) * 0.5)
        );
        // bottom vertex
        mesh_data.add_vertex(
            center + Vec3::new(x, -height / 2.0, z),
            Vec3::NEG_Y,
            Vec2::new((cos_a + 1.0) * 0.5, (sin_a + 1.0) * 0.5)
        );

        // side normal
        let side_normal = Vec3::new(cos_a, 0.0, sin_a);
        // top
        mesh_data.add_vertex(
            center + Vec3::new(x, height / 2.0, z),
            side_normal,
            Vec2::new(i as f32 / segments as f32, 1.0)
        );
        // bottom
        mesh_data.add_vertex(
            center + Vec3::new(x, -height / 2.0, z),
            side_normal,
            Vec2::new(i as f32 / segments as f32, 0.0)
        );

        let next_i = (i + 1) % segments;
        let next_side_top = start_idx + 2 + next_i * 4 + 2;
        let next_side_bottom = start_idx + 2 + next_i * 4 + 3;

        // triangle on top of area
        mesh_data.indices.extend_from_slice(&[
            top_center,
            start_idx + 2 + i * 4,
            start_idx + 2 + next_i * 4,
        ]);

        // trianle on bottom of area
        mesh_data.indices.extend_from_slice(&[
            bottom_center,
            start_idx + 2 + next_i * 4 + 1,
            start_idx + 2 + i * 4 + 1,
        ]);

        // square of side area (two triangles)
        let curr_side_top = start_idx + 2 + i * 4 + 2;
        let curr_side_bottom = start_idx + 2 + i * 4 + 3;

        mesh_data.indices.extend_from_slice(&[
            curr_side_top,
            curr_side_bottom,
            next_side_top,
        ]);
        mesh_data.indices.extend_from_slice(&[
            next_side_top,
            curr_side_bottom,
            next_side_bottom,
        ]);
    }
}
use bevy::{prelude::*, render::mesh};
use crate::core::constants::*;

pub fn generate_detailed_pawn_mesh() -> Mesh {
    let mut mesh_data = MeshData::new();

    add_cylinder_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, 0.0, 0.0),
        PIECE_RADIUS,
        PAWN_HEIGHT * 0.2,
        8
    );

    add_tapered_cylinder_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, PAWN_HEIGHT * 0.2, 0.0),
        PIECE_RADIUS * 0.8,
        PIECE_RADIUS * 0.6,
        PAWN_HEIGHT * 0.5,
        8
    );

    add_sphere_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, PAWN_HEIGHT * 0.8, 0.0),
        PIECE_RADIUS * 0.4,
        6,
        4,
    );

    mesh_data.to_mesh()
}

pub fn generate_detailed_rook_mesh() -> Mesh {
    let mut mesh_data = MeshData::new();

    add_cylinder_to_mesh(
        &mut mesh_data,
        Vec3::ZERO,
        PIECE_RADIUS * 0.9,
        ROOK_HEIGHT * 0.3,
        8
    );

    add_cylinder_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, ROOK_HEIGHT * 0.3, 0.0),
        PIECE_RADIUS * 0.7,
        ROOK_HEIGHT * 0.5,
        8
    );

    let wall_height = ROOK_HEIGHT * 0.2;
    let wall_thickness = 0.08;
    let wall_y = ROOK_HEIGHT * 0.8;

    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::PI * 0.25;
        let x = angle.cos() * PIECE_RADIUS * 0.8;
        let z = angle.sin() * PIECE_RADIUS * 0.8;

        if i % 2 == 0 {
            // High part
            add_box_to_mesh(
                &mut mesh_data,
                Vec3::new(x, wall_y + wall_height * 0.5, z),
                Vec3::new(wall_thickness, wall_height, wall_thickness)
            );
        } else {
            // Low part
            add_box_to_mesh(
                &mut mesh_data,
                Vec3::new(z, wall_y + wall_height * 0.3, z),
                Vec3::new(wall_thickness, wall_height * 0.6, wall_thickness)
            );
        }
    }

    mesh_data.to_mesh()
}

pub fn generate_detailed_knight_mesh() -> Mesh {
    let mut mesh_data = MeshData::new();

    add_cylinder_to_mesh(
        &mut mesh_data,
        Vec3::ZERO,
        PIECE_RADIUS * 0.8,
        KNIGHT_HEIGHT * 0.3,
        8
    );

    add_cylinder_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, KNIGHT_HEIGHT * 0.4, PIECE_RADIUS * 0.2),
        PIECE_RADIUS * 0.4,
        KNIGHT_HEIGHT * 0.3,
        6
    );

    add_elongated_sphere_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, KNIGHT_HEIGHT * 0.7, PIECE_RADIUS * 0.4),
        Vec3::new(PIECE_RADIUS * 0.3, PIECE_RADIUS * 0.25, PIECE_RADIUS * 0.4),
        6,
        4
    );

    add_box_to_mesh(
        &mut mesh_data,
        Vec3::new(PIECE_RADIUS * 0.2, KNIGHT_HEIGHT * 0.8, PIECE_RADIUS * 0.3),
        Vec3::new(0.05, 0.15, 0.03)
    );
    add_box_to_mesh(
        &mut mesh_data,
        Vec3::new(-PIECE_RADIUS * 0.2, KNIGHT_HEIGHT * 0.8, PIECE_RADIUS * 0.3),
        Vec3::new(0.05, 0.15, 0.03)
    );

    mesh_data.to_mesh()
}

pub fn generate_detailed_bishop_mesh() -> Mesh {
    let mut mesh_data = MeshData::new();

    add_cylinder_to_mesh(
        &mut mesh_data,
        Vec3::ZERO,
        PIECE_RADIUS * 0.8,
        BISHOP_HEIGHT * 0.25,
        8
    );

    add_tapered_cylinder_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, BISHOP_HEIGHT * 0.25, 0.0),
        PIECE_RADIUS * 0.7,
        PIECE_RADIUS * 0.3,
        BISHOP_HEIGHT * 0.6,
        8
    );

    add_sphere_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, BISHOP_HEIGHT * 0.9, 0.0),
        PIECE_RADIUS * 0.25,
        6,
        4
    );

    add_box_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, BISHOP_HEIGHT * 0.95, 0.0),
        Vec3::new(0.03, 0.1, 0.15)
    );
    add_box_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, BISHOP_HEIGHT * 0.95, 0.0),
        Vec3::new(0.15, 0.1, 0.03)
    );

    mesh_data.to_mesh()
}

pub fn generate_detailed_queen_mesh() -> Mesh {
    let mut mesh_data = MeshData::new();

    add_cylinder_to_mesh(
        &mut mesh_data,
        Vec3::ZERO,
        PIECE_RADIUS * 0.9,
        QUEEN_HEIGHT * 0.3,
        8
    );

    add_tapered_cylinder_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, QUEEN_HEIGHT * 0.3, 0.0),
        PIECE_RADIUS * 0.8,
        PIECE_RADIUS * 0.5,
        QUEEN_HEIGHT * 0.4,
        8
    );

    add_cylinder_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, QUEEN_HEIGHT * 0.7, 0.0),
        PIECE_RADIUS * 0.6,
        QUEEN_HEIGHT * 0.1,
        8
    );

    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::PI * 0.25;
        let x = angle.cos() * PIECE_RADIUS * 0.55;
        let z = angle.sin() * PIECE_RADIUS * 0.55;
        let height = if 1 % 2 == 0 { 0.25 } else { 0.15 };

        add_tapered_cylinder_to_mesh(
            &mut mesh_data,
            Vec3::new(x, QUEEN_HEIGHT * 0.8, z),
            0.04,
            0.02,
            QUEEN_HEIGHT * height,
            4
        );
    }

    mesh_data.to_mesh()
}

pub fn generate_detailed_king_mesh() -> Mesh {
    let mut mesh_data = MeshData::new();

    add_cylinder_to_mesh(
        &mut mesh_data,
        Vec3::ZERO,
        PIECE_RADIUS,
        KING_HEIGHT * 0.3,
        8
    );

    add_tapered_cylinder_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, KING_HEIGHT * 0.3, 0.0),
        PIECE_RADIUS * 0.9,
        PIECE_RADIUS * 0.6,
        KING_HEIGHT * 0.5,
        8
    );

    add_cylinder_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, KING_HEIGHT * 0.8, 0.0),
        PIECE_RADIUS * 0.7,
        KING_HEIGHT * 0.1,
        8
    );

    add_box_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, KING_HEIGHT * 0.95, 0.0),
        Vec3::new(0.05, KING_HEIGHT * 0.2, 0.05)
    );

    add_box_to_mesh(
        &mut mesh_data,
        Vec3::new(0.0, KING_HEIGHT * 1.0, 0.0),
        Vec3::new(0.15, 0.05, 0.05)
    );

    mesh_data.to_mesh()
}

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

// 以下すべて実装が長くなりそうなため簡略化
fn add_tapered_cylinder_to_mesh(
    mesh_data: &mut MeshData, 
    center: Vec3, 
    bottom_radius: f32, 
    top_radius: f32, 
    height: f32, 
    segments: u32
) {
    // テーパー付きシリンダーの実装
    // 簡略化のため、通常のシリンダーとして実装
    add_cylinder_to_mesh(mesh_data, center, (bottom_radius + top_radius) * 0.5, height, segments);
}

fn add_sphere_to_mesh(mesh_data: &mut MeshData, center: Vec3, radius: f32, rings: u32, sectors: u32) {
    // 球体の実装を簡略化
    let sphere_mesh = Sphere::new(radius).mesh().uv(sectors, rings);
    // ここでは簡略化のため、基本的な球体として追加
    // 実際の実装では sphere_mesh の頂点データを mesh_data に追加する
}

fn add_elongated_sphere_to_mesh(mesh_data: &mut MeshData, center: Vec3, scale: Vec3, rings: u32, sectors: u32) {
    // 拡張された球体の実装
    add_sphere_to_mesh(mesh_data, center, scale.x.max(scale.y).max(scale.z), rings, sectors);
}

fn add_box_to_mesh(mesh_data: &mut MeshData, center: Vec3, size: Vec3) {
    // 箱の実装を簡略化
    let box_mesh = Mesh::from(Cuboid::new(size.x, size.y, size.z));
    // 実際の実装では box_mesh の頂点データを mesh_data に追加する
}
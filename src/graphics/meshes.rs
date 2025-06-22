use bevy::{prelude::*, render::mesh::{Indices, VertexAttributeValues}};
use crate::core::constants::*;

#[derive(Resource)]
pub struct ChessMeshes {
    pub board_square: Handle<Mesh>,
    pub pawn: Handle<Mesh>,
    pub rook: Handle<Mesh>,
    pub knight: Handle<Mesh>,
    pub bishop: Handle<Mesh>,
    pub queen: Handle<Mesh>,
    pub king: Handle<Mesh>,
    pub highlight_circle: Handle<Mesh>,
    pub highlight_square: Handle<Mesh>,
}

impl Default for ChessMeshes {
    fn default() -> Self {
        Self {
            board_square: Handle::default(),
            pawn: Handle::default(),
            rook: Handle::default(),
            knight: Handle::default(),
            bishop: Handle::default(),
            queen: Handle::default(),
            king: Handle::default(),
            highlight_circle: Handle::default(),
            highlight_square: Handle::default(),
        }
    }
}

impl ChessMeshes {
    pub fn initialize(&mut self, meshes: &mut Assets<Mesh>) {
        self.board_square = meshes.add(
            Mesh::from(Cuboid::new(SQUARE_SIZE, BOARD_THICKNESS, SQUARE_SIZE))
        );

        self.pawn = meshes.add(generate_pawn_mesh());
        self.rook = meshes.add(generate_rook_mesh());
        self.knight = meshes.add(generate_knight_mesh());
        self.bishop = meshes.add(generate_bishop_mesh());
        self.queen = meshes.add(generate_queen_mesh());
        self.king = meshes.add(generate_king_mesh());

        self.highlight_circle = meshes.add(
            Mesh::from(Cylinder::new(SQUARE_SIZE * 0.4, 0.05))
        );

        self.highlight_square = meshes.add(
            Mesh::from(Cuboid::new(SQUARE_SIZE * 0.9, 0.02, SQUARE_SIZE * 0.9))
        );
    }

    pub fn get_piece_mesh(&self, piece_type: crate::game::pieces::PieceType) -> Handle<Mesh> {
        match piece_type {
            crate::game::pieces::PieceType::Pawn => self.pawn.clone(),
            crate::game::pieces::PieceType::Rook => self.rook.clone(),
            crate::game::pieces::PieceType::Knight => self.knight.clone(),
            crate::game::pieces::PieceType::Bishop => self.bishop.clone(),
            crate::game::pieces::PieceType::Queen => self.queen.clone(),
            crate::game::pieces::PieceType::King => self.king.clone(),
        }
    }
}

fn generate_pawn_mesh() -> Mesh {
    let mut mesh = Mesh::from(Sphere::new(PIECE_RADIUS * 0.7).mesh().uv(8, 6));

    let base = Mesh::from(Cylinder::new(PIECE_RADIUS, PAWN_HEIGHT * 0.3));
    merge_meshes(&mut mesh, base, Vec3::new(0.0, -PAWN_HEIGHT * 0.3, 0.0));

    mesh
}

fn generate_rook_mesh() -> Mesh {
    let mut mesh = Mesh::from(Cylinder::new(PIECE_RADIUS * 0.8, ROOK_HEIGHT * 0.8));

    for i in 0..4 {
        let angle = i as f32 * std::f32::consts::PI * 0.5;
        let x = angle.cos() * PIECE_RADIUS * 0.9;
        let z = angle.sin() * PIECE_RADIUS * 0.9;
        let tower = Mesh::from(Cuboid::new(0.1, ROOK_HEIGHT * 0.3, 0.1));
        merge_meshes(&mut mesh, tower, Vec3::new(x, ROOK_HEIGHT * 0.6, z));
    }

    mesh
}

fn generate_knight_mesh() -> Mesh {
    let mut mesh = Mesh::from(Cylinder::new(PIECE_RADIUS * 0.7, KNIGHT_HEIGHT * 0.6));

    let head = Mesh::from(Sphere::new(PIECE_RADIUS * 0.5).mesh().uv(6, 4));
    merge_meshes(&mut mesh, head, Vec3::new(0.0, KNIGHT_HEIGHT * 0.4, PIECE_RADIUS * 0.3));

    mesh
}

fn generate_bishop_mesh() -> Mesh {
    let mut mesh = Mesh::from(Cylinder::new(PIECE_RADIUS * 0.7, BISHOP_HEIGHT * 0.7));

    let top = Mesh::from(Sphere::new(PIECE_RADIUS * 0.3).mesh().uv(6, 4));
    merge_meshes(&mut mesh, top, Vec3::new(0.0, BISHOP_HEIGHT * 0.6, 0.0));

    mesh
}

fn generate_queen_mesh() -> Mesh {
    let mut mesh = Mesh::from(Cylinder::new(PIECE_RADIUS * 0.8, QUEEN_HEIGHT * 0.7));

    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::PI * 0.25;
        let x = angle.cos() * PIECE_RADIUS * 0.9;
        let z = angle.sin() * PIECE_RADIUS * 0.9;
        let height = if i % 2 == 0 { 0.3 } else { 0.2 };
        let spike = Mesh::from(Cylinder::new(0.05, QUEEN_HEIGHT * height));
        merge_meshes(&mut mesh, spike, Vec3::new(x, QUEEN_HEIGHT * 0.7, z));
    }

    mesh
}

fn generate_king_mesh() -> Mesh {
    let mut mesh = Mesh::from(Cylinder::new(PIECE_RADIUS * 0.9, KING_HEIGHT * 0.8));

    let cross_v = Mesh::from(Cuboid::new(0.1, KING_HEIGHT * 0.3, 0.05));
    let cross_h = Mesh::from(Cuboid::new(0.05, 0.1, KING_HEIGHT * 0.15));
    merge_meshes(&mut mesh, cross_v, Vec3::new(0.0, KING_HEIGHT * 0.8, 0.0));
    merge_meshes(&mut mesh, cross_h, Vec3::new(0.0, KING_HEIGHT * 0.85, 0.0));
    
    mesh
}

fn merge_meshes(base: &mut Mesh, additional: Mesh, offset: Vec3) {
    let base_vertex_cnt = base.count_vertices();

    // 頂点のマージ（POSITION, NORMAL, UV_0）
    for attr in [Mesh::ATTRIBUTE_POSITION, Mesh::ATTRIBUTE_NORMAL, Mesh::ATTRIBUTE_UV_0] {
        if let (Some(VertexAttributeValues::Float32x3(base_vals)),
                Some(VertexAttributeValues::Float32x3(additional_vals))) =
            (base.attribute_mut(attr).cloned(), additional.attribute(attr))
        {
            let mut merged = base_vals;
            for mut pos in additional_vals.clone() {
                // offset を POSITION にのみ適用
                if attr.name == Mesh::ATTRIBUTE_POSITION.name {
                    pos[0] += offset.x;
                    pos[1] += offset.y;
                    pos[2] += offset.z;
                }
                merged.push(pos);
            }
            base.insert_attribute(attr, VertexAttributeValues::Float32x3(merged));
        }
        // UVs (Vec2)
        else if let (Some(VertexAttributeValues::Float32x2(base_vals)),
                    Some(VertexAttributeValues::Float32x2(additional_vals))) =
            (base.attribute_mut(attr).cloned(), additional.attribute(attr))
        {
            let mut merged = base_vals;
            merged.extend_from_slice(additional_vals);
            base.insert_attribute(attr, VertexAttributeValues::Float32x2(merged));
        }
    }

    if let Some(Indices::U32(mut base_indices)) = base.indices_mut().cloned() {
        if let Some(Indices::U32(additional_indices)) = additional.indices() {
            let adjusted_indices = additional_indices
                .iter()
                .map(|i| i + base_vertex_cnt as u32)
                .collect::<Vec<_>>();
            base_indices.extend(adjusted_indices);
            base.insert_indices(Indices::U32(base_indices));
        }
    } else {
        panic!("Non-indexed meshes are not supported in merge_meshes");
    }
}

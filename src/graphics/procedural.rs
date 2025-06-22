use bevy::prelude::*;
use crate::core::constants::*;

struct MeshData {
    verticles: Vec<Vec3>,
    normals: Vec<Vec3>,
    uvs: Vec<Vec2>,
    indices: Vec<u32>,
}

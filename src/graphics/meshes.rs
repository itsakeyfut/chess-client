use bevy::prelude::*;
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

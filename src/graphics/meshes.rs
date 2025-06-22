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

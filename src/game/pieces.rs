use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn opposite(self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }

    pub fn to_string(self) -> &'static str {
        match self {
            PieceColor::White => "white",
            PieceColor::Black => "black",
        }
    }

    pub fn is_uppercase(self) -> bool {
        matches!(self, PieceColor::White)
    }
}

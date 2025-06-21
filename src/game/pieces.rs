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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceType {
    pub fn to_fen_char(self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Rook => 'r',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        }
    }

    pub fn from_fen_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            'p' => Some(PieceType::Pawn),
            'r' => Some(PieceType::Rook),
            'n' => Some(PieceType::Knight),
            'b' => Some(PieceType::Bishop),
            'q' => Some(PieceType::Queen),
            'k' => Some(PieceType::King),
            _ => None,
        }
    }

    pub fn value(self) -> u32 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 0,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            PieceType::Pawn => "Pawn",
            PieceType::Knight => "Knight",
            PieceType::Bishop => "Bishop",
            PieceType::Rook => "Rook",
            PieceType::Queen => "Queen",
            PieceType::King => "King",
        }
    }

    pub fn unicode_symbol(self, color: PieceColor) -> char {
        match (self, color) {
            (PieceType::King, PieceColor::White) => '♔',
            (PieceType::Queen, PieceColor::White) => '♕',
            (PieceType::Rook, PieceColor::White) => '♖',
            (PieceType::Bishop, PieceColor::White) => '♗',
            (PieceType::Knight, PieceColor::White) => '♘',
            (PieceType::Pawn, PieceColor::White) => '♙',
            (PieceType::King, PieceColor::Black) => '♚',
            (PieceType::Queen, PieceColor::Black) => '♛',
            (PieceType::Rook, PieceColor::Black) => '♜',
            (PieceType::Bishop, PieceColor::Black) => '♝',
            (PieceType::Knight, PieceColor::Black) => '♞',
            (PieceType::Pawn, PieceColor::Black) => '♟',
        }
    }

    pub fn can_promote_to(self) -> bool {
        matches!(self, PieceType::Queen | PieceType::Rook | PieceType::Bishop | PieceType::Knight)
    }
}

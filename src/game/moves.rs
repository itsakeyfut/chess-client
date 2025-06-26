use bevy::prelude::*;
use crate::{BoardPosition, PieceColor, PieceType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: BoardPosition,
    pub to: BoardPosition,
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub captured_piece: Option<PieceType>,
    pub promotion: Option<PieceType>,
    pub is_castling: bool,
    pub is_en_passant: bool,
    pub check_status: MoveCheckStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveCheckStatus {
    None,
    Check,
    Checkmate,
}

impl Move {
    pub fn new(from: BoardPosition, to: BoardPosition, piece_type: PieceType, color: PieceColor) -> Self {
        Self {
            from,
            to,
            piece_type,
            color,
            captured_piece: None,
            promotion: None,
            is_castling: false,
            is_en_passant: false,
            check_status: MoveCheckStatus::None,
        }
    }

    pub fn with_capture(mut self, captured_piece: PieceType) -> Self {
        self.captured_piece = Some(captured_piece);
        self
    }
}
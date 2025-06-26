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
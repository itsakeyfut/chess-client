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

    pub fn with_promotion(mut self, promotion: PieceType) -> Self {
        self.promotion = Some(promotion);
        self
    }

    pub fn with_castling(mut self) -> Self {
        self.is_castling = true;
        self
    }

    pub fn with_en_passant(mut self) -> Self {
        self.is_en_passant = true;
        self
    }

    pub fn with_check_status(mut self, status: MoveCheckStatus) -> Self {
        self.check_status = status;
        self
    }

    pub fn to_algebraic_notation(&self) -> String {
        let mut notation = String::new();

        if self.is_castling {
            if self.to.file > self.from.file {
                return "O-O".to_string(); // Kingside
            } else {
                return "O-O-O".to_string(); // Queenside
            }
        }

        if self.piece_type != PieceType::Pawn {
            notation.push(self.piece_type.to_fen_char().to_ascii_uppercase());
        }

        if self.captured_piece.is_some() {
            if self.piece_type == PieceType::Pawn {
                notation.push((b'a' + self.from.file) as char);
            }
            notation.push('x');
        }

        // destination
        notation.push_str(&self.to.to_algebraic());

        if let Some(promotion) = self.promotion {
            notation.push('=');
            notation.push(promotion.to_fen_char().to_ascii_uppercase());
        }

        match self.check_status {
            MoveCheckStatus::Check => notation.push('+'),
            MoveCheckStatus::Checkmate => notation.push('#'),
            MoveCheckStatus::None => {},
        }

        notation
    }
}
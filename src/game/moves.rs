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

fn generate_pawn_moves(
    board: &crate::game::ChessBoard,
    pieces: &Query<&crate::game::pieces::ChessPiece>,
    position: BoardPosition,
    color: PieceColor,
) -> Vec<BoardPosition> {
    let mut moves = Vec::new();
    let direction = if color == PieceColor::White { 1 } else { -1 };
    let start_rank = if color == PieceColor::White { 1 } else { 6 };

    // Move forward
    if let Some(forward_pos) = position.offset(0, direction) {
        if board.is_empty(forward_pos) {
            moves.push(forward_pos);

            if position.rank == start_rank {
                if let Some(double_forward) = position.offset(0, direction * 2) {
                    if board.is_empty(double_forward) {
                        moves.push(double_forward);
                    }
                }
            }
        }
    }

    // Diagonal attack
    for file_offset in [-1, 1] {
        if let Some(attack_pos) = position.offset(file_offset, direction) {
            if let Some(target_entity) = board.get_piece_at(attack_pos) {
                if let Ok(target_piece) = pieces.get(target_entity) {
                    if target_piece.color != color {
                        moves.push(attack_pos);
                    }
                }
            }
        }
    }

    moves
}

fn generate_rook_moves(
    board: &crate::game::ChessBoard,
    pieces: &Query<&crate::game::pieces::ChessPiece>,
    position: BoardPosition,
    color: PieceColor,
) -> Vec<BoardPosition> {
    let mut moves = Vec::new();
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for (file_dir, rank_dir) in directions {
        let mut current_pos = position;

        while let Some(next_pos) = current_pos.offset(file_dir, rank_dir) {
            if let Some(piece_entity) = board.get_piece_at(next_pos) {
                if let Ok(piece) = pieces.get(piece_entity) {
                    if piece.color != color {
                        moves.push(next_pos);
                    }
                }
                break; // stop because the next position is already occupied
            } else {
                moves.push(next_pos); // brank
                current_pos = next_pos;
            }
        }
    }

    moves
}

fn generate_knight_moves(
    board: &crate::game::ChessBoard,
    pieces: &Query<&crate::game::pieces::ChessPiece>,
    position: BoardPosition,
    color: PieceColor,
) -> Vec<BoardPosition> {
    let mut moves = Vec::new();
    let knight_moves = [
        (2, 1), (2, -1), (-2, 1), (-2, -1),
        (1, 2), (1, -2), (-1, 2), (-1, -2),
    ];

    for (file_offset, rank_offset) in knight_moves {
        if let Some(target_pos) = position.offset(file_offset, rank_offset) {
            if let Some(piece_entity) = board.get_piece_at(target_pos) {
                if let Ok(piece) = pieces.get(piece_entity) {
                    if piece.color != color {
                        moves.push(target_pos);
                    }
                }
            } else {
                moves.push(target_pos); // brank
            }
        }
    }

    moves
}

fn generate_bishop_moves(
    board: &crate::game::ChessBoard,
    pieces: &Query<&crate::game::pieces::ChessPiece>,
    position: BoardPosition,
    color: PieceColor,
) -> Vec<BoardPosition> {
    let mut moves = Vec::new();
    let directions = [(1, 1), (1, -1), (-1, 1), (-1, 1)];

    for (file_dir, rank_dir) in directions {
        let mut current_pos = position;

        while let Some(next_pos) = current_pos.offset(file_dir, rank_dir) {
            if let Some(piece_entity) = board.get_piece_at(next_pos) {
                if let Ok(piece) = pieces.get(piece_entity) {
                    if piece.color != color {
                        moves.push(next_pos);
                    }
                }
                break; // stop because the next position is already occupied
            } else {
                moves.push(next_pos); // brank
                current_pos = next_pos;
            }
        }
    }

    moves
}

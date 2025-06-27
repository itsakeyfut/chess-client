use bevy::prelude::*;
use crate::{game::{
    pieces::{ChessPiece, PieceColor},
    BoardPosition
}, PieceType};

pub fn is_valid_move(
    board: &crate::game::ChessBoard,
    pieces: &Query<&ChessPiece>,
    from: BoardPosition,
    to: BoardPosition,
    current_player: PieceColor
) -> Result<(), MoveError> {
    if !from.is_valid() || !to.is_valid() {
        return Err(MoveError::InvalidPosition);
    }

    if from == to {
        return Err(MoveError::SamePosition);
    }

    let piece_entity = board.get_piece_at(from)
        .ok_or(MoveError::NoPieceAtSource)?;

    let piece = pieces.get(piece_entity)
        .map_err(|_| MoveError::InvalidPiece)?;

    if piece.color != current_player {
        return Err(MoveError::WrongPlayerPiece);
    }

    if let Some(target_entity) = board.get_piece_at(to) {
        if let Ok(target_piece) = pieces.get(target_entity) {
            if target_piece.color == current_player {
                return Err(MoveError::OwnPieceBlocking);
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveError {
    InvalidPosition,
    SamePosition,
    NoPieceAtSource,
    InvalidPiece,
    WrongPlayerPiece,
    OwnPieceBlocking,
    IllegalMove,
    KingInCheck,
    PathBlocked,
}

fn is_legal_piece_move(
    piece: &ChessPiece,
    from: BoardPosition,
    to: BoardPosition,
    board: &crate::game::ChessBoard,
    pieces: &Query<&ChessPiece>,
) -> bool {
    match piece.piece_type {
        PieceType::Pawn => is_legal_pawn_move(piece, from, to, board, pieces),
        PieceType::Rook => is_legal_rook_move(from, to, board),
        PieceType::Knight => is_legal_knight_move(from, to),
        PieceType::Bishop => is_legal_bishop_move(from, to, board),
        PieceType::Queen => is_legal_queen_move(from, to, board),
        PieceType::King => is_legal_king_move(from, to),
    }
}

fn is_legal_pawn_move(
    piece: &ChessPiece,
    from: BoardPosition,
    to: BoardPosition,
    board: &crate::game::ChessBoard,
    pieces: &Query<&ChessPiece>
) -> bool {
    let direction = if piece.color == PieceColor::White { 1 } else { -1 };
    let start_rank = if piece.color == PieceColor::White { 1 } else { 6 };

    let file_diff = to.file as i8 - from.file as i8;
    let rank_diff = to.rank as i8 - from.rank as i8;

    // Move Forward
    if file_diff == 0 {
        if rank_diff == direction && board.is_empty(to) {
            return true; // 1 step
        }
        if rank_diff == direction * 2 && from.rank == start_rank && board.is_empty(to) {
            return true; // 2 steps
        }
    }
    // Diagonal attack
    else if file_diff.abs() == 1 && rank_diff == direction {
        if let Some(target_entity) = board.get_piece_at(to) {
            if let Ok(target_piece) = pieces.get(target_entity) {
                return target_piece.color != piece.color;
            }
        }
    }

    false
}

fn is_legal_rook_move(from: BoardPosition, to: BoardPosition, board: &crate::game::ChessBoard) -> bool {
    if !from.is_orthogonal_to(&to) {
        return false;
    }

    is_path_clear(from, to, board)
}

fn is_legal_knight_move(from: BoardPosition, to: BoardPosition) -> bool {
    let file_diff = (to.file as i8 - from.file as i8).abs();
    let rank_diff = (to.rank as i8 - from.rank as i8).abs();

    (file_diff == 2 && rank_diff == 1) || (file_diff == 1 && rank_diff == 2)
}

fn is_legal_bishop_move(from: BoardPosition, to: BoardPosition, board: &crate::game::ChessBoard) -> bool {
    if !from.is_diagonal_to(&to) {
        return false;
    }

    is_path_clear(from, to, board)
}

fn is_legal_queen_move(from: BoardPosition, to: BoardPosition, board: &crate::game::ChessBoard) -> bool {
    if from.is_orthogonal_to(&to) || from.is_diagonal_to(&to) {
        return is_path_clear(from, to, board);
    }
    false
}

fn is_legal_king_move(from: BoardPosition, to: BoardPosition) -> bool {
    let file_diff = (to.file as i8 - from.file as i8).abs();
    let rank_diff = (to.rank as i8 - from.rank as i8).abs();

    file_diff <= 1 && rank_diff <= 1
}

fn is_path_clear(from: BoardPosition, to: BoardPosition, board: &crate::game::ChessBoard) -> bool {
    let positions_between = from.positions_between(&to);

    for pos in positions_between {
        if !board.is_empty(pos) {
            return false;
        }
    }

    true
}
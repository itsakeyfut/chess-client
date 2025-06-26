use bevy::prelude::*;
use crate::game::{
    pieces::{ChessPiece, PieceColor},
    BoardPosition
};

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
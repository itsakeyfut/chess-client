use bevy::prelude::*;
use crate::{game::ChessPiece, BoardPosition};

pub fn is_valid_move(
    board: &crate::game::ChessBoard,
    pieces: &Query<&ChessPiece>,
    from: BoardPosition,
    to: BoardPosition,
) -> Result<(), MoveError> {
    if !from.is_valid() || !to.is_valid() {
        return Err(MoveError::InvalidPosition);
    }

    if from == to {
        return Err(MoveError::InvalidPosition);
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
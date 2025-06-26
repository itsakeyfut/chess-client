use bevy::prelude::*;

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
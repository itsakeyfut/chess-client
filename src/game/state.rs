use bevy::prelude::*;
use crate::PieceColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    Opening,
    Middlegame,
    Endgame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    InProgress,
    Check,
    Checkmate,
    Stalemate,
    Draw,
    Resigned,
    Timeout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckStatus {
    None,
    Check(PieceColor),
    Checkmate(PieceColor),
}

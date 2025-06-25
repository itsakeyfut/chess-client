use bevy::prelude::*;
use crate::{BoardPosition, PieceColor};

#[derive(Resource)]
pub struct GameStateResource {
    pub current_player: PieceColor,
    pub move_count: u32,
    pub game_phase: GamePhase,
    pub game_status: GameStatus,
    pub check_status: CheckStatus,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<BoardPosition>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub block_queenside: bool,
}

impl CastlingRights {
    pub fn all() -> Self {
        Self {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            block_queenside: true,
        }
    }

    pub fn none() -> Self {
        Self {
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            block_queenside: false,
        }
    }
}
use bevy::prelude::*;
use crate::{BoardPosition, PieceColor, PieceType};

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

impl Default for GameStateResource {
    fn default() -> Self {
        Self {
            current_player: PieceColor::White,
            move_count: 0,
            game_phase: GamePhase::Opening,
            game_status: GameStatus::InProgress,
            check_status: CheckStatus::None,
            castling_rights: CastlingRights::all(),
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
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

#[derive(Resource, Default)]
pub struct MoveHistory {
    pub moves: Vec<ChessMove>,
    pub positions: Vec<String>, // FEN strings
}

#[derive(Debug, Clone)]
pub struct ChessMove {
    pub from: BoardPosition,
    pub to: BoardPosition,
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
    pub captured_piece: Option<PieceType>,
    pub promotion: Option<PieceType>,
    pub is_castling: bool,
    pub is_en_passant: bool,
    pub notation: String,
    pub timestamp: f64,
}

#[derive(Resource, Default)]
pub struct SelectionState {
    pub selected_piece: Option<Entity>,
    pub selected_position: Option<BoardPosition>,
    pub legal_moves: Vec<BoardPosition>,
    pub highlighted_squares: Vec<Entity>,
}

impl SelectionState {
    pub fn clear(&mut self) {
        self.selected_piece = None;
        self.selected_position = None;
        self.legal_moves.clear();
        self.highlighted_squares.clear();
    }

    pub fn select_piece(&mut self, entity: Entity, position: BoardPosition) {
        self.selected_piece = Some(entity);
        self.selected_position = Some(position);
    }

    pub fn is_selected(&self, entity: Entity) -> bool {
        self.selected_piece == Some(entity)
    }
}

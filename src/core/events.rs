use bevy::prelude::*;
use crate::game::board::BoardPosition;

#[derive(Event)]
pub struct PieceSelectedEvent {
    pub entity: Entity,
    pub position: BoardPosition,
    pub piece_type: crate::game::PieceType,
    pub color: crate::game::pieces::PieceColor,
}

#[derive(Event)]
pub struct MovePieceEvent {
    pub from: BoardPosition,
    pub to: BoardPosition,
    pub promotion: Option<crate::game::pieces::PieceType>,
    pub is_player_move: bool,
}

#[derive(Debug, Clone)]
pub enum NetworkMessage {
    Connect {
        player_name: String,
        clinet_version: String,
    },
    PlayerLeft {
        player_id: String,
        reason: String,
    },
    Pong {
        timestamp: u64,
    },
    Error {
        error_code: String,
        message: String,
        details: Option<String>,
    },
    Success {
        message: String,
    },
}
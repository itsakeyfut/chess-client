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

#[derive(Event)]
pub struct SendNetworkMessageEvent {
    pub message: NetworkMessage,
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

#[derive(Debug, Clone)]
pub enum CameraAction {
    Zoom(f32),
    Rotate { delta_z: f32, delta_y: f32 },
    SetPerspective(crate::game::pieces::PieceColor),
    Reset,
    ToggleAutoRotate,
    SetTarget(Vec3),
    SmoothMoveTo {
        position: Vec3,
        target: Vec3,
        duration: f32,
    },
}

#[derive(Debug, Clone)]
pub enum GameAction {
    StartNewGame,
    JoinGame(String),
    LeaveGame,
    PauseGame,
    ResumeGame,
    RestartGame,
    OfferDraw,
    Resign,
    RequestUndo,
    ShowLegalMoves(bool),
    ToggleCoordinates,
    FlipBoard,
    SaveGame,
    LoadGame,
    ExportPGN,
    ImportPGN(String),
}

#[derive(Debug, Clone)]
pub enum AudioAction {
    PlaySFX {
        sound_name: String,
        volume: Option<f32>,
    },
    PlayMusic {
        music_name: String,
        loop_music: bool,
        fade_in: Option<f32>,
    },
    StopMusic {
        fade_out: Option<f32>,
    },
    SetMasterVolume(f32),
    SetSFXVolume(f32),
    SetMusicVolume(f32),
    ToggleSound,
    ToggleMusic,
}

#[derive(Debug, Clone)]
pub enum SettingType {
    Graphics,
    Audio,
    Input,
    Network,
    Gameplay,
}

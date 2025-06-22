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

#[derive(Event)]
pub struct CameraControlEvent {
    pub action: CameraAction,
}

#[derive(Event)]
pub struct GameActionEvent {
    pub action: GameAction,
}

#[derive(Event)]
pub struct AudioEvent {
    pub action: AudioAction,
}

#[derive(Event)]
pub struct SettingsChangedEvent {
    pub settings_type: SettingType,
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

#[derive(Debug, Clone)]
pub struct TimeControl {
    pub initial_time_seconds: u32,
    pub increment_seconds: u32,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum ChatMessageType {
    Game,
    Global,
    System,
    Private,
}

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub server_name: String,
    pub version: String,
    pub max_players: u32,
    pub current_players: u32,
    pub features: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub id: String,
    pub name: String,
    pub rating: u32,
    pub games_played: u32,
    pub win_rate: f32,
    pub is_online: bool,
    pub status: PlayerStatus,
}

#[derive(Debug, Clone)]
pub enum PlayerStatus {
    Online,
    Away,
    InGame,
    Offline,
}

#[derive(Debug, Clone)]
pub struct GameInfo {
    pub id: String,
    pub white_player: Option<PlayerInfo>,
    pub black_player: Option<PlayerInfo>,
    pub status: GameStatus,
    pub time_control: Option<TimeControl>,
    pub move_count: u32,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub enum GameStatus {
    Waiting,
    Active,
    Finished,
}

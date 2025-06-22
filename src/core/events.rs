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

#[derive(Debug, Clone)]
pub struct GameStateSnapshot {
    pub board_fen: Vec<String>,
    pub move_history: Vec<String>,
    pub white_player: Option<PlayerInfo>,
    pub black_player: Option<PlayerInfo>,
    pub current_player: crate::game::pieces::PieceColor,
    pub move_count: u32,
    pub is_check: bool,
    pub game_result: Option<GameResult>,
    pub time_control: Option<TimeControl>,
    pub white_time_remaing: Option<f32>,
    pub black_time_remaing: Option<f32>,
    pub last_move: Option<(BoardPosition, BoardPosition)>,
}

#[derive(Debug, Clone)]
pub enum GameResult {
    WhiteWins(GameEndReason),
    BlackWins(GameEndReason),
    Draw(DrawReason),
}

#[derive(Debug, Clone)]
pub enum GameEndReason {
    Checkmate,
    Resignation,
    Timeout,
    Disconnection,
}

#[derive(Debug, Clone)]
pub enum DrawReason {
    Stalemate,
    InsufficientMaterial,
    ThreefoldRepetition,
    FiftyMoveRule,
    Agreement,
    Timeout,
}

#[derive(Debug, Clone)]
pub struct ErrorInfo {
    pub error_code: String,
    pub message: String,
    pub timestamp: u64,
    pub details: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NotificationInfo {
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub duration: Option<f32>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
    GameEvent,
}

#[derive(Event)]
pub struct AnimationEvent {
    pub entity: Entity,
    pub animation_type: AnimationType,
}

#[derive(Debug, Clone)]
pub enum AnimationType {
    PieceMove {
        from: Vec3,
        to: Vec3,
        duration: f32,
        ease_type: EaseType,
    },
    PieceCapture {
        capture_position: Vec3,
        duration: f32,
    },
    PiecePromotion {
        position: Vec3,
        new_piece_type: crate::game::pieces::PieceType,
        duration: f32,
    },
    BoardHighlight {
        positions: Vec<BoardPosition>,
        highlight_type: HighlightType,
        duration: Option<f32>,
    },
    CameraMove {
        target_position: Vec3,
        target_look_at: Vec3,
        duration: f32,
        ease_type: EaseType,
    },
    UIFadeIn {
        duration: f32,
    },
    UIFadeOut {
        duration: f32,
    },
}

#[derive(Debug, Clone)]
pub enum EaseType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
}

#[derive(Debug, Clone)]
pub enum HighlightType {
    Selected,
    LegalMove,
    LastMove,
    Check,
    Capture,
    Threat,
}

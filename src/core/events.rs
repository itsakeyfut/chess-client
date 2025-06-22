use bevy::prelude::*;
use crate::game::board::BoardPosition;

#[derive(Event)]
pub struct PieceSelectedEvent {
    pub entity: Entity,
    pub position: BoardPosition,
    pub piece_type: crate::game::pieces::PieceType,
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

#[derive(Event)]
pub struct NetworkEvent {
    pub event_type: NetworkEventType,
}

#[derive(Debug, Clone)]
pub enum NetworkEventType {
    Connected,
    Disconnected { reason: String },
    ReconnectAttempt { attempt: u32 },
    MessageReceived { message: ServerResponse },
    MessageSent { message: ClientRequest },
    Error { error: NetworkError },
}

#[derive(Debug, Clone)]
pub struct NetworkError {
    pub code: u32,
    pub message: String,
    pub is_recoverable: bool,
}

#[derive(Debug, Clone)]
pub enum NetworkMessage {
    Connect {
        player_name: String,
        client_version: String,
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
    Rotate { delta_x: f32, delta_y: f32 },
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
    AcceptDraw,
    DeclineDraw,
    Resign,
    RequestUndo,
    AccpetUndo,
    DeclineUndo,
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
    pub white_time_remaining: Option<f32>,
    pub black_time_remaining: Option<f32>,
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

#[derive(Event)]
pub struct InputEvent {
    pub input_type: InputType,
}

#[derive(Debug)]
pub enum InputType {
    MouseClick {
        position: Vec2,
        button: MouseButton,
        world_position: Option<Vec3>,
        board_position: Option<BoardPosition>,
    },
    MouseDrag {
        start: Vec2,
        current: Vec2,
        delta: Vec2,
        button: MouseButton,
    },
    MouseWheel {
        delta: f32,
    },
    KeyPress {
        key: KeyCode,
        modifiers: InputModifiers,
    },
    GamepadInput {
        gamepad: Gamepad,
        input: GamepadInput,
    },
}

#[derive(Debug, Clone)]
pub struct InputModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub super_key: bool,
}

#[derive(Debug, Clone)]
pub enum GamepadInput {
    Button(GamepadButton),
    Axis { axis: GamepadAxis, value: f32 },
}

#[derive(Event)]
pub struct PerformanceEvent {
    pub event_type: PerformanceEventType,
}

#[derive(Debug, Clone)]
pub enum PerformanceEventType {
    FrameTimeSpike { frame_time_ms: f32 },
    MemoryUsageHigh { usage_mb: f32 },
    LowFPS { fps: f32 },
    SystemPerformance {
        system_name: String,
        execution_time_ms: f32,
    },
}

#[derive(Event)]
pub struct DebugEvent {
    pub message: String,
    pub debug_type: DebugType,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum DebugType {
    Info,
    Warning,
    Error,
    Performance,
    Network,
    Gameplay,
}

impl PieceSelectedEvent {
    pub fn new(
        entity: Entity,
        position: BoardPosition,
        piece_type: crate::game::pieces::PieceType,
        color: crate::game::pieces::PieceColor,
    ) -> Self {
        Self {
            entity,
            position,
            piece_type,
            color,
        }
    }
}

impl MovePieceEvent {
    pub fn player_move(from: BoardPosition, to: BoardPosition) -> Self {
        Self {
            from,
            to,
            promotion: None,
            is_player_move: true,
        }
    }

    pub fn network_move(from: BoardPosition, to: BoardPosition) -> Self {
        Self {
            from,
            to,
            promotion: None,
            is_player_move: false,
        }
    }

    pub fn with_promotion(mut self, piece_type: crate::game::pieces::PieceType) -> Self {
        self.promotion = Some(piece_type);
        self
    }
}

impl AudioEvent {
    pub fn play_sound(sound_name: &str) -> Self {
        Self {
            action: AudioAction::PlaySFX {
                sound_name: sound_name.to_string(),
                volume: None,
            },
        }
    }

    pub fn play_sound_with_volume(sound_name: &str, volume: f32) -> Self {
        Self {
            action: AudioAction::PlaySFX {
                sound_name: sound_name.to_string(),
                volume: Some(volume),
            },
        }
    }

    pub fn play_music(music_name: &str, loop_music: bool) -> Self {
        Self {
            action: AudioAction::PlayMusic {
                music_name: music_name.to_string(),
                loop_music,
                fade_in: None,
            },
        }
    }

    pub fn stop_music() -> Self {
        Self {
            action: AudioAction::StopMusic { fade_out: None },
        }
    }
}

impl NotificationInfo {
    pub fn info(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            notification_type: NotificationType::Info,
            duration: Some(3.0),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn success(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            notification_type: NotificationType::Success,
            duration: Some(2.0),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn error(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            notification_type: NotificationType::Error,
            duration: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn game_event(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            notification_type: NotificationType::GameEvent,
            duration: Some(4.0),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl CameraControlEvent {
    pub fn zoom(delta: f32) -> Self {
        Self {
            action: CameraAction::Zoom(delta),
        }
    }

    pub fn rotate(delta_x: f32, delta_y: f32) -> Self {
        Self {
            action: CameraAction::Rotate { delta_x, delta_y },
        }
    }

    pub fn reset() -> Self {
        Self {
            action: CameraAction::Reset,
        }
    }

    pub fn set_perspective(color: crate::game::pieces::PieceColor) -> Self {
        Self {
            action: CameraAction::SetPerspective(color),
        }
    }
}

impl GameActionEvent {
    pub fn start_new_game() -> Self {
        Self {
            action: GameAction::StartNewGame,
        }
    }

    pub fn join_game(game_id: String) -> Self {
        Self {
            action: GameAction::JoinGame(game_id),
        }
    }

    pub fn offer_draw() -> Self {
        Self {
            action: GameAction::OfferDraw,
        }
    }

    pub fn resign() -> Self {
        Self {
            action: GameAction::Resign,
        }
    }
}

impl DebugEvent {
    pub fn new(message: String, debug_type: DebugType) -> Self {
        Self {
            message,
            debug_type,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn info(message: String) -> Self {
        Self::new(message, DebugType::Info)
    }

    pub fn warning(message: String) -> Self {
        Self::new(message, DebugType::Warning)
    }

    pub fn error(message: String) -> Self {
        Self::new(message, DebugType::Error)
    }

    pub fn performance(message: String) -> Self {
        Self::new(message, DebugType::Performance)
    }

    pub fn network(message: String) -> Self {
        Self::new(message, DebugType::Network)
    }

    pub fn gameplay(message: String) -> Self {
        Self::new(message, DebugType::Gameplay)
    }
}

#[derive(Debug, Clone)]
pub enum ServerResponse {
    Connected {
        player_id: String,
        session_id: String,
        server_info: ServerInfo,
    },
    Authenticated {
        player_info: PlayerInfo,
    },
    GameCreated {
        game_id: String,
        player_color: crate::game::pieces::PieceColor,
    },
    GameJoined {
        game_id: String,
        player_color: crate::game::pieces::PieceColor,
        opponent_info: Option<PlayerInfo>,
        game_state: GameStateSnapshot,
    },
    MoveUpdate {
        from: BoardPosition,
        to: BoardPosition,
        promotion: Option<crate::game::pieces::PieceType>,
        move_number: u32,
        time_remaining: Option<(f32, f32)>,
    },
    GameStateUpdate {
        game_state: GameStateSnapshot,
    },
    GameOver {
        result: GameResult,
    },
    DrawOffered {
        from_player: String,
    },
    DrawResponse {
        accepted: bool,
    },
    UndoOffered {
        from_player: String,
        moves_count: u32,
    },
    UndoResponse {
        accepted: bool,
    },
    ChatMessage {
        from_player: String,
        message: String,
        message_type: ChatMessageType,
        timestamp: u64,
    },
    GameList {
        games: Vec<GameInfo>,
    },
    PlayerList {
        players: Vec<PlayerInfo>,
    },
    Error {
        message: String,
        error_code: Option<u32>,
    },
    Pong,
}

#[derive(Debug, Clone)]
pub enum ClientRequest {
    CreateGame {
        time_control: Option<TimeControl>,
        is_private: bool,
    },
    JoinGame {
        game_id: String,
        password: Option<String>,
    },
    MakeMove { 
        from: BoardPosition, 
        to: BoardPosition,
        promotion: Option<crate::game::pieces::PieceType>,
    },
    OfferDraw,
    AcceptDraw,
    DeclineDraw,
    Resign,
    RequestUndo,
    AcceptUndo,
    DeclineUndo,
    SendChatMessage {
        message: String,
        message_type: ChatMessageType,
    },
    Disconnect {
        reason: Option<String>,
    },
    Ping,
    GetGameList,
    GetPlayerList,
}


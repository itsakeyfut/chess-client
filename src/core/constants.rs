use bevy::prelude::*;

// Board
pub const BOARD_SIZE: f32 = 8.0;
pub const SQUARE_SIZE: f32 = 1.0;
pub const BOARD_THICKNESS: f32 = 0.2;

// Piece
pub const PAWN_HEIGHT: f32 = 1.0;
pub const ROOK_HEIGHT: f32 = 1.2;
pub const KNIGHT_HEIGHT: f32 = 1.3;
pub const BISHOP_HEIGHT: f32 = 1.4;
pub const QUEEN_HEIGHT: f32 = 1.6;
pub const KING_HEIGHT: f32 = 1.8;

pub const PIECE_RADIUS: f32 = 0.35;
pub const PIECE_BASE_SCALE: f32 = 0.8;

// Animation
pub const PIECE_MOVE_DURATION: f32 = 0.5;
pub const PIECE_CAPTURE_DURATION: f32 = 0.3;
pub const PIECE_HOVER_HEIGHT: f32 = 0.2;
pub const BOARD_FLIP_DURATION: f32 = 1.0;
pub const CAMERA_MOVE_DURATION: f32 = 1.5;
pub const UI_FADE_DURATION: f32 = 0.25;

// Camera
pub const CAMERA_MIN_DISTANCE: f32 = 5.0;
pub const CAMERA_MAX_DISTANCE: f32 = 25.0;
pub const CAMERA_DEFAULT_DISTANCE: f32 = 12.0;
pub const CAMERA_DEFAULT_ANGLE_X: f32 = -45.0;
pub const CAMERA_DEFAULT_ANGLE_Y: f32 = 0.0;
pub const CAMERA_ROTATION_SPEED: f32 = 2.0;
pub const CAMERA_ZOOM_SPEED: f32 = 1.0;
pub const CAMERA_SMOOTH_FACTOR: f32 = 5.0;

// Lighting
pub const AMBIENT_LIGHT_STRENGTH: f32 = 0.3;
pub const DIRECTIONAL_LIGHT_STRENGTH: f32 = 0.8;
pub const DIRECTIONAL_LIGHT_ANGLE: f32 = -45.0;

// Material
pub const WHITE_PIECE_COLOR: Color = Color::srgb(0.95, 0.95, 0.92);
pub const BLACK_PIECE_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const LIGHT_SQUARE_COLOR: Color = Color::srgb(0.93, 0.85, 0.73);
pub const DARK_SQUARE_COLOR: Color = Color::srgb(0.65, 0.42, 0.24);

// Highlight Color
pub const SELECTED_COLOR: Color = Color::srgb(0.2, 0.8, 0.2);
pub const LEGAL_MOVE_COLOR: Color = Color::srgba(0.2, 0.2, 0.8, 0.7);
pub const LAST_MOVE_COLOR: Color = Color::srgba(0.8, 0.8, 0.2, 0.5);
pub const CHECK_COLOR: Color = Color::srgb(0.9, 0.1, 0.1);
pub const CAPTURE_COLOR: Color = Color::srgba(0.9, 0.3, 0.1, 0.8);
pub const THREAT_COLOR: Color = Color::srgba(0.9, 0.6, 0.1, 0.6);

// Effect
pub const PARTICLE_COUNT: u32 = 50;
pub const EXPLOSION_DURATION: f32 = 1.0;
pub const SPARKLE_DURATION: f32 = 2.0;
pub const GLOW_INTENSITY: f32 = 1.5;

// UI
pub const UI_FONT_SIZE_SMALL: f32 = 14.0;
pub const UI_FONT_SIZE_MEDIUM: f32 = 18.0;
pub const UI_FONT_SIZE_LARGE: f32 = 24.0;
pub const UI_FONT_SIZE_TITLE: f32 = 32.0;

pub const UI_MARGIN_SMALL: f32 = 4.0;
pub const UI_MARGIN_MEDIUM: f32 = 8.0;
pub const UI_MARGIN_LARGE: f32 = 16.0;

pub const UI_BUTTON_HEIGHT: f32 = 40.0;
pub const UI_PANEL_BORDER_RADIUS: f32 = 8.0;

// UI Color
pub const UI_BACKGROUND_COLOR: Color = Color::srgba(0.1, 0.1, 0.1, 0.9);
pub const UI_PANEL_COLOR: Color = Color::srgba(0.2, 0.2, 0.2, 0.95);
pub const UI_BUTTON_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
pub const UI_BUTTON_HOVER_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);
pub const UI_BUTTON_PRESSED_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
pub const UI_TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const UI_TEXT_DISABLED_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
pub const UI_ACCENT_COLOR: Color = Color::srgb(0.2, 0.6, 0.9);
pub const UI_SUCCESS_COLOR: Color = Color::srgb(0.2, 0.8, 0.2);
pub const UI_WARNING_COLOR: Color = Color::srgb(0.9, 0.7, 0.1);
pub const UI_ERROR_COLOR: Color = Color::srgb(0.9, 0.2, 0.2);

// Networking
pub const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1:8080";
pub const CONNECTION_TIMEOUT_SECONDS: f32 = 10.0;
pub const RECONNECT_ATTEMPTS: u32 = 3;
pub const PING_INTERVAL_SECONDS: f32 = 30.0;
pub const NETWORK_BUFFER_SIZE: usize = 8192;

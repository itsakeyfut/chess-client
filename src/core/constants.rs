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

// Audio
pub const DEFAULT_MASTER_VOLUME: f32 = 0.8;
pub const DEFAULT_SFX_VOLUME: f32 = 0.7;
pub const DEFAULT_MUSIC_VOLUME: f32 = 0.5;
pub const AUDIO_FADE_DURATION: f32 = 1.0;

// Performance
pub const TARGET_FPS: f32 = 60.0;
pub const LOW_FPS_THRESHOLD: f32 = 30.0;
pub const HIGH_MEMORY_THRESHOLD_MB: f32 = 1024.0;
pub const FRAME_TIME_SPIKE_THRESHOLD_MS: f32 = 50.0;

// Graphics Quality
pub mod graphics_quality {
    pub const LOW_MSAA_SAMPLES: u32 = 1;
    pub const MEDIUM_MSAA_SAMPLES: u32 = 2;
    pub const HIGH_MSAA_SAMPLES: u32 = 4;
    pub const ULTRA_MSAA_SAMPLES: u32 = 8;

    pub const LOW_SHADOW_RESOLUTION: u32 = 512;
    pub const MEDIUM_SHADOW_RESOLUTION: u32 = 1024;
    pub const HIGH_SHADOW_RESOLUTION: u32 = 2048;
    pub const ULTRA_SHADOW_RESOLUTION: u32 = 4096;

    pub const LOW_REFLECTION_RESOLUTION: u32 = 256;
    pub const MEDIUM_REFLECTION_RESOLUTION: u32 = 512;
    pub const HIGH_REFLECTION_RESOLUTION: u32 = 1024;
    pub const ULTRA_REFLECTION_RESOLUTION: u32 = 2048;
}

// File path
pub const CONFIG_FILE_PATH: &str = "config/settings.toml";
pub const SAVE_GAME_DIR: &str = "saves/";
pub const SCREENSHOT_DIR: &str = "screenshots/";
pub const LOG_FILE_PATH: &str = "logs/application.log";

// Asset path
pub mod asset_paths {
    pub const FONTS_DIR: &str = "fonts/";
    pub const TEXTURES_DIR: &str = "textures/";
    pub const MODELS_DIR: &str = "models/";
    pub const SOUNDS_DIR: &str = "sounds/";
    pub const MUSIC_DIR: &str = "music/";
    pub const SHADERS_DIR: &str = "shaders/";

    // Fonts
    pub const DEFAULT_FONT: &str = "fonts/FiraSans-Bold.ttf";
    pub const MONO_FONT: &str = "fonts/FiraMono-Medium.ttf";

    // Textures
    pub const WOOD_TEXTURE: &str = "textures/wood.png";
    pub const MARBLE_TEXTURE: &str = "textures/marble.png";
    pub const METAL_TEXTURE: &str = "textures/metal.png";

    // Sounds
    pub const PIECE_MOVE_SOUND: &str = "sounds/piece_move.ogg";
    pub const PIECE_CAPTURE_SOUND: &str = "sounds/piece_capture.ogg";
    pub const CHECK_SOUND: &str = "sounds/check_sound.ogg";
    pub const CHECKMATE_SOUND: &str = "sounds/checkmate.ogg";
    pub const DRAW_SOUND: &str = "sounds/draw.ogg";
    pub const CASTLE_SOUND: &str = "sounds/castle.ogg";
    pub const PROMOTION_SOUND: &str = "sounds/promotion.ogg";
    pub const NOTIFICATION_SOUND: &str = "sounds/notification.ogg";
    pub const ERROR_SOUND: &str = "sounds/error.ogg";
    pub const BUTTON_CLICK_SOUND: &str = "sounds/button_click.ogg";

    // Music
    pub const MENU_MUSIC: &str = "music/menu_theme.ogg";
    pub const GAME_MUSIC: &str = "music/game_theme.ogg";
    pub const VICTORY_MUSIC: &str = "music/victory.ogg";
    pub const DEFEAT_MUSIC: &str = "music/defeat.ogg";
}

// Layer (Rendering Order)
pub mod render_layers {
    pub const BOARD: f32 = 0.0;
    pub const PIECES: f32 = 1.0;
    pub const HIGHLIGHTS: f32 = 0.5;
    pub const EFFECT: f32 = 2.0;
    pub const UI_BACKGRUOND: f32 = 10.0;
    pub const UI_CONTENT: f32 = 11.0;
    pub const UI_OVERLAY: f32 = 12.0;
    pub const TOOLTIP: f32 = 15.0;
    pub const DEBUG: f32 = 20.0;
}

// Physics
pub mod physics {
    pub const GRAVITY: f32 = -9.81;
    pub const PIECE_MASS: f32 = 1.0;
    pub const BOARD_FRICTION: f32 = 0.7;
    pub const PIECE_RESTITUTION: f32 = 0.3;
}

// Timing
pub mod timing {
    pub const DOUBLE_CLICK_TIME: f32 = 0.3;
    pub const LONG_PRESS_TIME: f32 = 0.8;
    pub const TOOLTIP_DELAY: f32 = 1.0;
    pub const AUTO_SAVE_INTERVAL: f32 = 60.0; // 1min
    pub const PING_TIMEOUT: f32 = 5.0;
}

// Game rule
pub mod game_rules {
    pub const FIFTY_MOVE_LIMIT: u32 = 50;
    pub const THREEFOLD_REPETITION_LIMIT: u32 = 3;
    pub const INSUFFICIENT_MATERIAL_PIECES: u32 = 3;
}

// Debug
#[cfg(debug_assertions)]
pub mod debug {
    pub const SHOW_FPS_BY_DEFAULT: bool = true;
    pub const SHOW_WIREFRAME_BY_DEFAULT: bool = true;
    pub const LOG_NETWORK_MESSAGES: bool = true;
    pub const LOG_PERFORMANCE: bool = true;
    pub const ENABLE_HOT_RELOAD: bool = true;
}

#[cfg(not(debug_assertions))]
pub mod debug {
    pub const SHOW_FPS_BY_DEFAULT: bool = false;
    pub const SHOW_WIREFRAME_BY_DEFAULT: bool = false;
    pub const LOG_NETWORK_MESSAGES: bool = false;
    pub const LOG_PERFORMANCE: bool = false;
    pub const ENABLE_HOT_RELOAD: bool = false;
}

// Version info
pub const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CLIENT_NAME: &str = "Chess 3D Client";
pub const PROTOCOL_VERSION: &str = "1.0";

// Window settings
pub const DEFAULT_WINDOW_WIDTH: f32 = 1280.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 720.0;
pub const MIN_WINDOW_WIDTH: f32 = 800.0;
pub const MIN_WINDOW_HEIGHT: f32 = 600.0;

// Inputs
pub const MOUSE_SENSITIVITY_MIN: f32 = 0.1;
pub const MOUSE_SENSITIVITY_MAX: f32 = 5.0;
pub const MOUSE_SENSITIVITY_DEFAULT: f32 = 1.0;
pub const DRAG_THRESHOLD: f32 = 5.0;

pub fn board_position_to_world(file: u8, rank: u8) -> Vec3 {
    Vec3::new(
        (file as f32 - 3.5) * SQUARE_SIZE,
        0.0,
        (rank as f32 - 3.5) * SQUARE_SIZE,
    )
}

pub fn world_to_board_position(world_pos: Vec3) -> Option<(u8, u8)> {
    let file = ((world_pos.x / SQUARE_SIZE + 3.5).round() as i32).clamp(0, 7) as u8;
    let rank = ((world_pos.z / SQUARE_SIZE + 3.5).round() as i32).clamp(0, 7) as u8;

    if file < 8 && rank < 8 {
        Some((file, rank))
    } else {
        None
    }
}

pub fn piece_height_by_type(piece_type: crate::game::pieces::PieceType) -> f32 {
    match piece_type {
        crate::game::pieces::PieceType::Pawn => PAWN_HEIGHT,
        crate::game::pieces::PieceType::Rook => ROOK_HEIGHT,
        crate::game::pieces::PieceType::Knight => KNIGHT_HEIGHT,
        crate::game::pieces::PieceType::Bishop => BISHOP_HEIGHT,
        crate::game::pieces::PieceType::Queen => QUEEN_HEIGHT,
        crate::game::pieces::PieceType::King => KING_HEIGHT,
    }
}

pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let a = a.to_srgba();
    let b = b.to_srgba();

    Color::srgba(
        a.red + (b.red - a.red) * t,
        a.green + (b.green - a.green) * t,
        a.blue + (b.blue - a.blue) * t,
        a.alpha + (b.alpha - a.alpha) * t,
    )
}

// pub fn lerp_color(a: Color, b: Color, t: f32) -> Color {
//     a.mix(&b, t)
// }

pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

pub fn ease_out_bounce(t: f32) -> f32 {
    const N1: f32 = 7.5625;
    const D1: f32 = 2.75;

    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        let t = t - 1.5 / D1;
        N1 * t * t + 0.75
    } else if t < 2.5 / D1 {
        let t = t - 2.25 / D1;
        N1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / D1;
        N1 * t * t + 0.984375
    }
}

pub fn ease_out_elastic(t: f32) -> f32 {
    const C4: f32 = (2.0 * std::f32::consts::PI) / 3.0;

    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin() + 1.0
    }
}

pub fn scale_color(color: Color, scalar: f32) -> Color {
    let srgba = color.to_srgba();
    Color::srgba(
        srgba.red * scalar,
        srgba.green * scalar,
        srgba.blue * scalar,
        srgba.alpha
    )
}

pub fn scale_color_linear(color: Color, scalar: f32) -> LinearRgba {
    let linear = color.to_linear();
    LinearRgba {
        red: linear.red * scalar,
        green: linear.green * scalar,
        blue: linear.blue * scalar,
        alpha: linear.alpha,
    }
}

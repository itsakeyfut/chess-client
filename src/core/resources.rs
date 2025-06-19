use bevy::prelude::*;

#[derive(Resource)]
pub struct GameSettings {
    pub server_address: String,
    pub player_name: String,
    pub graphics_quality: GraphicsQuality,
    pub camera_sensitivity: f32,
    pub auto_rotate_board: bool,
    pub show_legal_moves: bool,
    pub show_coordinates: bool,
    pub animation_speed: f32,
    pub sound_enabled: bool,
    pub music_enabled: bool,
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            server_address: "127.0.0.1:8080".to_string(),
            player_name: "Player".to_string(),
            graphics_quality: GraphicsQuality::High,
            camera_sensitivity: 1.0,
            auto_rotate_board: true,
            show_legal_moves: true,
            show_coordinates: true,
            animation_speed: 1.0,
            sound_enabled: true,
            music_enabled: true,
            master_volume: 0.8,
            sfx_volume: 0.7,
            music_volume: 0.5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsQuality {
    Low,
    Medium,
    High,
    Ultra,
}

#[derive(Resource, Default)]
pub struct UIState {
    pub active_ui_screens: Vec<crate::core::states::UIState>,
    pub show_debug_info: bool,
    pub show_performance_info: bool,
    pub modal_open: bool,
    pub tooltip_text: Option<String>,
    pub status_message: Option<String>,
    pub last_error: Option<String>,
}
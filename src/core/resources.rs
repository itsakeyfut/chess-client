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
    pub status_message: String,
    pub last_error: Option<String>,
}

impl UIState {
    pub fn is_screen_active(&self, screen: crate::core::states::UIState) -> bool {
        self.active_ui_screens.contains(&screen)
    }

    pub fn show_screen(&mut self, screen: crate::core::states::UIState) {
        if !self.active_ui_screens.contains(&screen) {
            self.active_ui_screens.push(screen);
        }
    }

    pub fn hide_screen(&mut self, screen: crate::core::states::UIState) {
        self.active_ui_screens.retain(|&s| s != screen);
    }

    pub fn clear_all_screens(&mut self) {
        self.active_ui_screens.clear();
    }

    pub fn has_modal(&self) -> bool {
        self.modal_open || self.active_ui_screens.iter().any(|s| s.is_overlay())
    }

    pub fn set_status(&mut self, message: String) {
        self.status_message = message;
    }

    pub fn set_error(&mut self, error: String) {
        self.last_error = Some(error.clone());
        self.status_message = format!("Error: {}", error);
    }

    pub fn clear_error(&mut self) {
        self.last_error = None;
    }
}

#[derive(Resource)]
pub struct CameraController {
    pub distance: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub angle_x: f32,
    pub angle_y: f32,
    pub target: Vec3,
    pub is_white_perspective: bool,
    pub smooth_follow: bool,
    pub follow_speed: f32,
    pub rotation_speed: f32,
    pub zoom_speed: f32,
    pub auto_rotate: bool,
    pub auto_rotate_speed: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            distance: 12.0,
            min_distance: 5.0,
            max_distance: 25.0,
            angle_x: -45.0,
            angle_y: 0.0,
            target: Vec3::ZERO,
            is_white_perspective: true,
            smooth_follow: true,
            follow_speed: 5.0,
            rotation_speed: 1.0,
            zoom_speed: 1.0,
            auto_rotate: true,
            auto_rotate_speed: 0.1,
        }
    }
}
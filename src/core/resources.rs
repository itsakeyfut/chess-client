use bevy::{platform::collections::HashMap, prelude::*};

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

impl CameraController {
    pub fn get_camera_position(&self) -> Vec3 {
        let angle_x_rad = self.angle_x.to_radians();
        let angle_y_rad = self.angle_y.to_radians();

        let x = self.distance * angle_x_rad.cos() * angle_y_rad.sin();
        let y = self.distance * angle_x_rad.sin();
        let z = self.distance * angle_x_rad.cos() * angle_y_rad.cos();

        self.target + Vec3::new(x, y, z)
    }

    pub fn zoom(&mut self, delta: f32) {
        self.distance = (self.distance - delta * self.zoom_speed)
            .clamp(self.min_distance, self.max_distance);
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        self.angle_y += delta_x * self.rotation_speed;
        self.angle_x = (self.angle_x + delta_y * self.rotation_speed)
            .clamp(-89.0, 89.0); // Prevent gimbal lock
    }

    pub fn set_perspective(&mut self, is_white: bool) {
        self.is_white_perspective = is_white;
        if is_white {
            self.angle_y = 0.0;
        } else {
            self.angle_y = 180.0;
        }
    }

    pub fn reset_to_default(&mut self) {
        self.distance = 12.0;
        self.angle_x = -45.0;
        self.angle_y = if self.is_white_perspective { 0.0 } else { 180.0 };
        self.target = Vec3::ZERO;
    }
}

#[derive(Resource, Default)]
pub struct NetworkState {
    pub connection_status: ConnectionStatus,
    pub server_address: String,
    pub player_id: Option<String>,
    pub game_id: Option<String>,
    pub pind: u32,
    pub last_ping_time: f32,
    pub reconnect_attempts: u32,
    pub max_reconnect_attempts: u32,
    pub is_reconnecting: bool,
    pub connection_start_time: f32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Authenticated,
    InGame,
    Error,
    Reconnecting,
}

impl NetworkState {
    pub fn is_connected(&self) -> bool {
        matches!(
            self.connection_status,
            ConnectionStatus::Connected | ConnectionStatus::Authenticated | ConnectionStatus::InGame
        )
    }

    pub fn can_send_messages(&self) -> bool {
        matches!(
            self.connection_status,
            ConnectionStatus::Authenticated | ConnectionStatus::InGame
        )
    }

    pub fn start_connection(&mut self, address: String, time: f32) {
        self.server_address = address;
        self.connection_status = ConnectionStatus::Connecting;
        self.connection_start_time = time;
        self.reconnect_attempts = 0;
    }

    pub fn connection_established(&mut self, player_id: String) {
        self.connection_status = ConnectionStatus::Connected;
        self.player_id = Some(player_id);
    }

    pub fn authenticated(&mut self) {
        self.connection_status = ConnectionStatus::Authenticated;
    }

    pub fn join_game(&mut self, game_id: String) {
        self.connection_status = ConnectionStatus::InGame;
        self.game_id = Some(game_id);
    }

    pub fn disconenct(&mut self) {
        self.connection_status = ConnectionStatus::Disconnected;
        self.player_id = None;
        self.game_id = None;
        self.pind = 0;
        self.reconnect_attempts = 0;
        self.is_reconnecting = false;
    }

    pub fn connection_error(&mut self) {
        self.connection_status = ConnectionStatus::Error;
        self.reconnect_attempts += 1;
    }

    pub fn should_reconnect(&self) -> bool {
        self.connection_status == ConnectionStatus::Error &&
        self.reconnect_attempts < self.max_reconnect_attempts &&
        !self.is_reconnecting
    }
}

#[derive(Resource)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub sound_enabled: bool,
    pub music_enabled: bool,
    pub current_bgm: Option<String>,
    pub sfx_queue: Vec<String>,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.8,
            sfx_volume: 0.7,
            music_volume: 0.5,
            sound_enabled: true,
            music_enabled: true,
            current_bgm: None,
            sfx_queue: Vec::new(),
        }
    }
}

impl AudioSettings {
    pub fn get_effective_master_volume(&self) -> f32 {
        if self.sound_enabled {
            self.master_volume * self.sfx_volume
        } else {
            0.0
        }
    }

    pub fn get_effective_music_volume(&self) -> f32 {
        if self.music_enabled {
            self.master_volume * self.music_volume
        } else {
            0.0
        }
    }

    pub fn queue_sfx(&mut self, sound_name: String) {
        self.sfx_queue.push(sound_name);
    }

    pub fn dequeue_sfx(&mut self) -> Option<String> {
        if self.sfx_queue.is_empty() {
            None
        } else {
            Some(self.sfx_queue.remove(0))
        }
    }
}

#[derive(Resource, Default)]
pub struct PerformanceStats {
    pub fps: f32,
    pub frame_count: u32,
    pub total_time: f32,
    pub memory_usage_mb: f32,
    pub draw_calls: u32,
    pub entities_count: u32,
    pub systems_time: HashMap<String, f32>,
}

impl PerformanceStats {
    pub fn reset_frame_stats(&mut self) {
        self.draw_calls = 0;
    }

    pub fn add_system_time(&mut self, system_name: String, time: f32) {
        *self.systems_time.entry(system_name).or_insert(0.0) += time;
    }

    pub fn get_average_frame_time(&self) -> f32 {
        if self.fps > 0.0 {
            1000.0 / self.fps
        } else {
            0.0
        }
    }
}

#[derive(Resource)]
pub struct InputSettings {
    pub mouse_sensitivity: f32,
    pub keyboard_repeat_delay: f32,
    pub keyboard_repeat_rate: f32,
    pub double_click_time: f32,
    pub draw_threshold: f32,
    pub invert_mouse_y: bool,
    pub key_bindings: HashMap<String, Vec<KeyCode>>,
}
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Settings,
    Connecting,
    Lobby,
    InGame,
    GameOver,
    Paused,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum NetworkState {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Authenticated,
    InGame,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UIState {
    MainMenu,
    Settings,
    GameLobby,
    InGameUI,
    PauseMenu,
    ConnectionDialog,
    ErrorDialog,
    ConfirmDialog,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum InputState {
    #[default]
    Menu,
    Game,
    Camera,
    Disabled,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AudioState {
    #[default]
    Manu,
    Game,
    Victory,
    Defeat,
    Muted,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GraphicsState {
    Low,
    #[default]
    Medium,
    High,
    Ultra,
}

impl GameState {
    pub fn is_in_game(&self) -> bool {
        matches!(self, GameState::InGame | GameState::Paused)
    }

    pub fn is_menu(&self) -> bool {
        matches!(self, GameState::MainMenu | GameState::Settings | GameState::Lobby)
    }

    pub fn can_pause(&self) -> bool {
        matches!(self, GameState::InGame)
    }

    pub fn requires_network(&self) -> bool {
        matches!(self, GameState::Connecting | GameState::Lobby | GameState::InGame)
    }
}

impl NetworkState {
    pub fn is_connected(&self) -> bool {
        matches!(self, NetworkState::Connected | NetworkState::Authenticated | NetworkState::InGame)
    }

    pub fn can_start_game(&self) -> bool {
        matches!(self, NetworkState::Authenticated)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, NetworkState::Error)
    }
}

impl UIState {
    pub fn is_fullscreen(&self) -> bool {
        matches!(self, UIState::MainMenu | UIState::Settings | UIState::GameLobby)
    }

    pub fn is_overlay(&self) -> bool {
        matches!(self, UIState::PauseMenu | UIState::ConnectionDialog | UIState::ErrorDialog | UIState::ConfirmDialog)
    }

    pub fn is_game_ui(&self) -> bool {
        matches!(self, UIState::InGameUI)
    }
}

impl InputState {
    pub fn game_input_enabled(&self) -> bool {
        matches!(self, InputState::Game | InputState::Camera)
    }

    pub fn menu_input_enabled(&self) -> bool {
        matches!(self, InputState::Menu)
    }

    pub fn camera_enabled(&self) -> bool {
        matches!(self, InputState::Camera | InputState::Game)
    }
}

impl GraphicsState {
    pub fn msaa_samples(&self) -> u32 {
        match self {
            GraphicsState::Low => 1,
            GraphicsState::Medium => 2,
            GraphicsState::High => 3,
            GraphicsState::Ultra => 8,
        }
    }

    pub fn shadow_quality(&self) -> f32 {
        match self {
            GraphicsState::Low => 0.5,
            GraphicsState::Medium => 1.0,
            GraphicsState::High => 1.5,
            GraphicsState::Ultra => 2.0,
        }
    }

    pub fn lighting_quality(&self) -> f32 {
        match self {
            GraphicsState::Low => 0.7,
            GraphicsState::Medium => 1.0,
            GraphicsState::High => 1.3,
            GraphicsState::Ultra => 1.5,
        }
    }

    pub fn particle_density(&self) -> f32 {
        match self {
            GraphicsState::Low => 0.3,
            GraphicsState::Medium => 0.6,
            GraphicsState::High => 1.0,
            GraphicsState::Ultra => 1.5,
        }
    }
}
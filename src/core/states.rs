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
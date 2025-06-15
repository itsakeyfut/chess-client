use bevy::prelude::*;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Connecting,
    InGame,
    GameOver,
}
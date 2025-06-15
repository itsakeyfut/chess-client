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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
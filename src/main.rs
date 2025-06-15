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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoardPosition {
    pub file: u8,
    pub rank: u8,
}

impl BoardPosition {
    pub fn new(file: u8, rank: u8) -> Option<Self> {
        if file < 8 && rank < 8 {
            Some(Self { file, rank })
        } else {
            None
        }
    }

    pub fn from_algebraic(notation: &str) -> Option<Self> {
        let chars: Vec<char> = notation.chars().collect();
        if chars.len() != 2 {
            return None;
        }

        let file = match chars[0] {
            'a'..='h' => chars[0] as u8 - b'a',
            _ => return None,
        };

        let rank = match chars[1] {
            '1'..='8' => chars[1] as u8 - b'1',
            _ => return None,
        };

        Some(Self { file, rank })
    }

    pub fn to_algebraic(&self) -> String {
        format!(
            "{}{}",
            (b'a' + self.file) as char,
            (b'1' + self.rank) as char
        )
    }

    pub fn to_world_position(&self) -> Vec3 {
        Vec3::new(
            (self.file as f32 - 3.5) * 1.0,
            0.0,
            (self.rank as f32 - 3.5) * 1.0,
        )
    }

    pub fn from_world_position(world_pos: Vec3) -> Option<Self> {
        let file = ((world_pos.x + 3.5).round() as i32).clamp(0, 7) as u8;
        let rank = ((world_pos.z + 3.5).round() as i32).clamp(0, 7) as u8;
        Self::new(file, rank)
    }
}
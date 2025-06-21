use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
            'a'..='h' => chars[0] as u8 -b'a',
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
        use crate::core::constants::{SQUARE_SIZE, BOARD_THICKNESS};
        Vec3::new(
            (self.file as f32 - 3.5) * SQUARE_SIZE,
            BOARD_THICKNESS / 2.0,
            (self.rank as f32 - 3.5) * SQUARE_SIZE,
        )
    }

    pub fn from_world_position(world_pos: Vec3) -> Option<Self> {
        use crate::core::constants::SQUARE_SIZE;
        let file = ((world_pos.x / SQUARE_SIZE + 3.5).round() as i32).clamp(0, 7) as u8;
        let rank = ((world_pos.z / SQUARE_SIZE + 3.5).round() as i32).clamp(0, 7) as u8;
        Self::new(file, rank)
    }

    pub fn is_valid(&self) -> bool {
        self.file < 8 && self.rank < 8
    }

    pub fn distance_to(&self, other: &BoardPosition) -> f32 {
        let dx = (self.file as i8 - other.file as i8).abs() as f32;
        let dy = (self.rank as i8 - other.rank as i8).abs() as f32;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn manhattan_distance_to(&self, other: &BoardPosition) -> u8 {
        let dx = (self.file as i8 - other.file as i8).abs() as u8;
        let dy = (self.rank as i8 - other.rank as i8).abs() as u8;
        dx + dy
    }

    pub fn is_diagonal_to(&self, other: &BoardPosition) -> bool {
        let dx = (self.file as i8 - other.file as i8).abs();
        let dy = (self.rank as i8 - other.rank as i8).abs();
        dx == dy && dx > 0
    }

    pub fn is_orthogonal_to(&self, other: &BoardPosition) -> bool {
        (self.file == other.file && self.rank != other.rank) ||
        (self.file != other.file && self.rank == other.rank)
    }

    pub fn is_adjacent_to(&self, other: &BoardPosition) -> bool {
        let dx = (self.file as i8 - other.file as i8).abs();
        let dy = (self.rank as i8 - other.rank as i8).abs();
        dx <= 1 && dy <= 1 && (dx + dy > 0)
    }

    pub fn offset(&self, file_delta: i8, rank_delta: i8) -> Option<Self> {
        let new_file = self.file as i8 + file_delta;
        let new_rank = self.rank as i8 + rank_delta;

        if new_file >= 0 && new_file < 8 && new_rank >= 0 && new_rank < 8 {
            Some(Self {
                file: new_file as u8,
                rank: new_rank as u8,
            })
        } else {
            None
        }
    }

    pub fn is_light_square(&self) -> bool {
        (self.file + self.rank) % 2 == 0
    }

    pub fn direction_to(&self, other: &BoardPosition) -> (i8, i8) {
        let file_diff = other.file as i8 - self.file as i8;
        let rank_diff = other.rank as i8 - self.rank as i8;
        (file_diff.signum(), rank_diff.signum())
    }

    pub fn positions_between(&self, other: &BoardPosition) -> Vec<BoardPosition> {
        let mut positions = Vec::new();
        let (file_dir, rank_dir) = self.direction_to(other);

        if file_dir == 0 && rank_dir == 0 {
            return positions;
        }

        let mut curr = *self;
        loop {
            if let Some(next) = curr.offset(file_dir, rank_dir) {
                if next == *other {
                    break;
                }
                positions.push(next);
                curr = next;
            } else {
                break;
            }
        }

        positions
    }
}

#[derive(Resource)]
pub struct ChessBoard {
    pub squares: [[Option<Entity>; 8]; 8],
    pub square_entities: [[Entity; 8]; 8],
    pub is_flipped: bool,
}

impl Default for ChessBoard {
    fn default() -> Self {
        Self {
            squares: [[None; 8]; 8],
            square_entities: [[Entity::PLACEHOLDER; 8]; 8],
            is_flipped: false,
        }
    }
}

impl ChessBoard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_piece_at(&self, pos: BoardPosition) -> Option<Entity> {
        if pos.is_valid() {
            self.squares[pos.rank as usize][pos.file as usize]
        } else {
            None
        }
    }

    pub fn set_piece_at(&mut self, pos: BoardPosition, entity: Option<Entity>) {
        if pos.is_valid() {
            self.squares[pos.rank as usize][pos.file as usize] = entity;
        }
    }

    pub fn move_piece(&mut self, from: BoardPosition, to: BoardPosition) -> Option<Entity> {
        let captured_piece = self.get_piece_at(to);
        let moving_piece = self.get_piece_at(from);

        self.set_piece_at(from, None);
        self.set_piece_at(to, moving_piece);

        captured_piece
    }

    pub fn is_empty(&self, pos: BoardPosition) -> bool {
        self.get_piece_at(pos).is_none()
    }

    pub fn get_square_entity(&self, pos: BoardPosition) -> Option<Entity> {
        if pos.is_valid() {
            Some(self.square_entities[pos.rank as usize][pos.file as usize])
        } else {
            None
        }
    }

    pub fn set_square_entity(&mut self, pos: BoardPosition, entity: Entity) {
        if pos.is_valid() {
            self.square_entities[pos.rank as usize][pos.file as usize] = entity;
        }
    }

    pub fn flip_board(&mut self) {
        self.is_flipped = !self.is_flipped
    }

    pub fn clear(&mut self) {
        self.squares =  [[None; 8]; 8];
    }

    pub fn get_all_pieces(&self) -> Vec<(BoardPosition, Entity)> {
        let mut pieces = Vec::new();
        for rank in 0..8 {
            for file in 0..8 {
                let pos = BoardPosition::new(file, rank).unwrap();
                if let Some(entity) = self.get_piece_at(pos) {
                    pieces.push((pos, entity));
                }
            }
        }
        pieces
    }
}
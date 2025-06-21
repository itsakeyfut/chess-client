use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::BoardPosition;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn opposite(self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }

    pub fn to_string(self) -> &'static str {
        match self {
            PieceColor::White => "white",
            PieceColor::Black => "black",
        }
    }

    pub fn is_uppercase(self) -> bool {
        matches!(self, PieceColor::White)
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceType {
    pub fn to_fen_char(self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Rook => 'r',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        }
    }

    pub fn from_fen_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            'p' => Some(PieceType::Pawn),
            'r' => Some(PieceType::Rook),
            'n' => Some(PieceType::Knight),
            'b' => Some(PieceType::Bishop),
            'q' => Some(PieceType::Queen),
            'k' => Some(PieceType::King),
            _ => None,
        }
    }

    pub fn value(self) -> u32 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 0,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            PieceType::Pawn => "Pawn",
            PieceType::Knight => "Knight",
            PieceType::Bishop => "Bishop",
            PieceType::Rook => "Rook",
            PieceType::Queen => "Queen",
            PieceType::King => "King",
        }
    }

    pub fn unicode_symbol(self, color: PieceColor) -> char {
        match (self, color) {
            (PieceType::King, PieceColor::White) => '♔',
            (PieceType::Queen, PieceColor::White) => '♕',
            (PieceType::Rook, PieceColor::White) => '♖',
            (PieceType::Bishop, PieceColor::White) => '♗',
            (PieceType::Knight, PieceColor::White) => '♘',
            (PieceType::Pawn, PieceColor::White) => '♙',
            (PieceType::King, PieceColor::Black) => '♚',
            (PieceType::Queen, PieceColor::Black) => '♛',
            (PieceType::Rook, PieceColor::Black) => '♜',
            (PieceType::Bishop, PieceColor::Black) => '♝',
            (PieceType::Knight, PieceColor::Black) => '♞',
            (PieceType::Pawn, PieceColor::Black) => '♟',
        }
    }

    pub fn can_promote_to(self) -> bool {
        matches!(self, PieceType::Queen | PieceType::Rook | PieceType::Bishop | PieceType::Knight)
    }
}

#[derive(Component, Debug, Clone)]
pub struct ChessPiece {
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub position: BoardPosition,
    pub has_moved: bool,
    pub move_count: u32,
    pub last_moved_turn: Option<u32>,
}

impl ChessPiece {
    pub fn new(piece_type: PieceType, color: PieceColor, position: BoardPosition) -> Self {
        Self {
            piece_type,
            color,
            position,
            has_moved: false,
            move_count: 0,
            last_moved_turn: None,
        }
    }

    pub fn to_fen_char(&self) -> char {
        let base_char = self.piece_type.to_fen_char();
        if self.color.is_uppercase() {
            base_char.to_ascii_uppercase()
        } else {
            base_char
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.color.to_string(), self.piece_type.name())
    }

    pub fn mark_moved(&mut self, turn: u32) {
        self.has_moved = true;
        self.move_count += 1;
        self.last_moved_turn = Some(turn)
    }

    pub fn set_position(&mut self, new_position: BoardPosition) {
        self.position = new_position;
    }

    pub fn is_empty(&self, other_color: PieceColor) -> bool {
        self.color != other_color
    }

    pub fn is_ally(&self, other_color: PieceColor) -> bool {
        self.color == other_color
    }

    pub fn can_castle(&self) -> bool {
        !self.has_moved && matches!(self.piece_type, PieceType::King | PieceType::Rook)
    }

    pub fn can_be_captured_en_passant(&self, current_turn: u32) -> bool {
        self.piece_type == PieceType::Pawn &&
        self.move_count == 1 &&
        self.last_moved_turn == Some(current_turn - 1)
    }
}

#[derive(Component)]
pub struct Selected {
    pub selected_at: f32,
}

#[derive(Component)]
pub struct PieceAnimation {
    pub animation_type: PieceAnimationType,
    pub start_time: f32,
    pub duration: f32,
    pub start_position: Vec3,
    pub target_position: Vec3,
    pub ease_function: EaseFunction,
}

#[derive(Debug, Clone)]
pub enum PieceAnimationType {
    Move,
    Capture,
    Castle,
    Promotion,
    Hover,
    Return,
}

#[derive(Debug, Clone)]
pub enum EaseFunction {
    Linear,
    EaseInOut,
    EaseOut,
    Bounce,
}

impl PieceAnimation {
    pub fn new_move(start_pos: Vec3, target_pos: Vec3, time: f32) -> Self {
        Self {
            animation_type: PieceAnimationType::Move,
            start_time: time,
            duration: crate::core::constants::PIECE_MOVE_DURATION,
            start_position: start_pos,
            target_position: target_pos,
            ease_function: EaseFunction::EaseOut,
        }
    }

    pub fn new_capture(pos: Vec3, time: f32) -> Self {
        Self {
            animation_type: PieceAnimationType::Capture,
            start_time: time,
            duration: crate::core::constants::PIECE_CAPTURE_DURATION,
            start_position: pos,
            target_position: pos + Vec3::new(0.0, -2.0, 0.0),
            ease_function: EaseFunction::EaseInOut,
        }
    }

    pub fn new_hover(original_pos: Vec3, time: f32) -> Self {
        let hover_pos = original_pos + Vec3::new(0.0, crate::core::constants::PIECE_HOVER_HEIGHT, 0.0);
        Self {
            animation_type: PieceAnimationType::Hover,
            start_time: time,
            duration: 0.2,
            start_position: original_pos,
            target_position: hover_pos,
            ease_function: EaseFunction::EaseOut,
        }
    }

    pub fn is_complete(&self, current_time: f32) -> bool {
        current_time - self.start_time >= self.duration
    }

    pub fn progress(&self, current_time: f32) -> f32 {
        ((current_time - self.start_time) / self.duration).clamp(0.0, 1.0)
    }

    pub fn eased_progress(&self, current_time: f32) -> f32 {
        let t = self.progress(current_time);
        match self.ease_function {
            EaseFunction::Linear => t,
            EaseFunction::EaseInOut => crate::core::constants::ease_in_out_cubic(t),
            EaseFunction::EaseOut => 1.0 - (1.0 - t).powi(3),
            EaseFunction::Bounce => crate::core::constants::ease_out_bounce(t),
        }
    }

    pub fn current_position(&self, current_time: f32) -> Vec3 {
        let t = self.eased_progress(current_time);
        self.start_position.lerp(self.target_position, t)
    }
}

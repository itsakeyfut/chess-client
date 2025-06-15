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

#[derive(Component)]
pub struct ChessPiece {
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub position: BoardPosition,
    pub has_moved: bool,
}

#[derive(Component)]
pub struct BoardSquare {
    pub position: BoardPosition,
    pub is_light: bool,
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Highlighted {
    pub highlight_type: HighlightType,
}

#[derive(Debug, Clone, Copy)]
pub enum HighlightType {
    Selected,
    LegalMove,
    LastMove,
    Check,
}

#[derive(Resource, Default)]
pub struct ChessGame {
    pub board: [[Option<Entity>; 8]; 8],
    pub current_player: PieceColor,
    pub selected_piece: Option<Entity>,
    pub legal_moves: Vec<BoardPosition>,
    pub game_over: bool,
    pub last_move: Option<(BoardPosition, BoardPosition)>,
}

impl ChessGame {
    pub fn new() -> Self {
        Self {
            board: [[None; 8]; 8],
            current_player: PieceColor::White,
            selected_piece: None,
            legal_moves: Vec::new(),
            game_over: false,
            last_move: None,
        }
    }

    pub fn get_piece_at(&self, pos: BoardPosition) -> Option<Entity> {
        self.board[pos.rank as usize][pos.file as usize]
    }

    pub fn set_piece_at(&mut self, pos: BoardPosition, entity: Option<Entity>) {
        self.board[pos.rank as usize][pos.file as usize] = entity;
    }

    pub fn move_piece(&mut self, from: BoardPosition, to: BoardPosition) {
        let piece = self.get_piece_at(from);
        self.set_piece_at(from, None);
        self.set_piece_at(to, piece);
        self.last_move = Some((from, to));

        self.current_player = match self.current_player {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
    }
}
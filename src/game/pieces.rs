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

#[derive(Component)]
pub struct PieceEffect {
    pub effect_type: PieceEffectType,
    pub intensity: f32,
    pub start_time: f32,
    pub duration: Option<f32>,
}

#[derive(Debug, Clone)]
pub enum PieceEffectType {
    Glow { color: Color },
    Pulse { color: Color, frequency: f32 },
    Sparkle { particle_count: u32 },
    Outline { color: Color, thickness: f32 },
    Shadow { offset: Vec3, opacity: f32 },
}

pub fn spawn_piece(
    commands: &mut Commands,
    piece_type: PieceType,
    color: PieceColor,
    position: BoardPosition,
    meshes: &crate::graphics::ChessMeshes,
    materials: &crate::graphics::ChessMaterials,
) -> Entity {
    let world_position = position.to_world_position();
    let mesh_handle = meshes.get_piece_mesh(piece_type);
    let material_handle = materials.get_piece_material(color);

    commands.spawn((
        PbrBundle {
            mesh: mesh_handle,
            material: material_handle,
            transform: Transform::from_translation(world_position),
            ..default()
        },
        ChessPiece::new(piece_type, color, position),
        Name::new(format!("{} {} at {}", 
            color.to_string(), 
            piece_type.name(), 
            position.to_algebraic())),
    )).id()
}

fn get_piece_mesh(piece_type: PieceType, meshes: &crate::graphics::ChessMeshes) -> Handle<Mesh> {
    match piece_type {
        PieceType::Pawn => meshes.pawn.clone(),
        PieceType::Rook => meshes.rook.clone(),
        PieceType::Knight => meshes.knight.clone(),
        PieceType::Bishop => meshes.bishop.clone(),
        PieceType::Queen => meshes.queen.clone(),
        PieceType::King => meshes.king.clone(),
    }
}

fn get_piece_material(color: PieceColor, materials: &crate::graphics::ChessMaterials) -> Handle<StandardMaterial> {
    match color {
        PieceColor::White => materials.white_piece.clone(),
        PieceColor::Black => materials.black_piece.clone(),
    }
}

pub fn spawn_initial_pieces(
    mut commands: Commands,
    mut board: ResMut<crate::game::board::ChessBoard>,
    meshes: Res<crate::graphics::ChessMeshes>,
    materials: Res<crate::graphics::ChessMaterials>,
) {
    // 白の駒
    let white_pieces = [
        (PieceType::Rook, "a1"), (PieceType::Knight, "b1"), 
        (PieceType::Bishop, "c1"), (PieceType::Queen, "d1"),
        (PieceType::King, "e1"), (PieceType::Bishop, "f1"), 
        (PieceType::Knight, "g1"), (PieceType::Rook, "h1"),
    ];

    for (piece_type, pos_str) in white_pieces {
        let position = BoardPosition::from_algebraic(pos_str).unwrap();
        let entity = spawn_piece(
            &mut commands,
            piece_type,
            PieceColor::White,
            position,
            &meshes,
            &materials,
        );
        board.set_piece_at(position, Some(entity));
    }

    // 白のポーン
    for file in 0..8 {
        let position = BoardPosition::new(file, 1).unwrap();
        let entity = spawn_piece(
            &mut commands,
            PieceType::Pawn,
            PieceColor::White,
            position,
            &meshes,
            &materials,
        );
        board.set_piece_at(position, Some(entity));
    }

    // 黒の駒
    let black_pieces = [
        (PieceType::Rook, "a8"), (PieceType::Knight, "b8"), 
        (PieceType::Bishop, "c8"), (PieceType::Queen, "d8"),
        (PieceType::King, "e8"), (PieceType::Bishop, "f8"), 
        (PieceType::Knight, "g8"), (PieceType::Rook, "h8"),
    ];

    for (piece_type, pos_str) in black_pieces {
        let position = BoardPosition::from_algebraic(pos_str).unwrap();
        let entity = spawn_piece(
            &mut commands,
            piece_type,
            PieceColor::Black,
            position,
            &meshes,
            &materials,
        );
        board.set_piece_at(position, Some(entity));
    }

    // 黒のポーン
    for file in 0..8 {
        let position = BoardPosition::new(file, 6).unwrap();
        let entity = spawn_piece(
            &mut commands,
            PieceType::Pawn,
            PieceColor::Black,
            position,
            &meshes,
            &materials,
        );
        board.set_piece_at(position, Some(entity));
    }
}

pub fn handle_piece_selection(
    mut commands: Commands,
    mut selected_pieces: Query<Entity, With<Selected>>,
    pieces: Query<(Entity, &ChessPiece)>,
    input_events: EventReader<crate::core::events::InputEvent>,
    time: Res<Time>,
) {
    for event in input_events.read() {
        if let crate::core::events::InputType::MouseClick { board_position: Some(pos), .. } = &event.input_type {
            // 既存の選択を解除
            for entity in selected_pieces.iter_mut() {
                commands.entity(entity).remove::<Selected>();
            }

            // クリックされた位置の駒を選択
            for (entity, piece) in pieces.iter() {
                if piece.position == *pos {
                    commands.entity(entity).insert(Selected {
                        selected_at: time.elapsed_secs(),
                    });
                    break;
                }
            }
        }
    }
}

pub fn animate_pieces(
    mut commands: Commands,
    mut pieces: Query<(Entity, &mut Transform, &PieceAnimation)>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    for (entity, mut transform, animation) in pieces.iter_mut() {
        if animation.is_complete(current_time) {
            // アニメーション完了
            transform.translation = animation.target_position;
            commands.entity(entity).remove::<PieceAnimation>();
            
            // キャプチャアニメーションの場合、駒を削除
            if matches!(animation.animation_type, PieceAnimationType::Capture) {
                commands.entity(entity).despawn();
            }
        } else {
            // アニメーション中
            transform.translation = animation.current_position(current_time);
        }
    }
}

pub fn move_piece(
    mut commands: Commands,
    mut pieces: Query<(Entity, &mut ChessPiece, &mut Transform)>,
    mut board: ResMut<crate::game::board::ChessBoard>,
    time: Res<Time>,
    from: BoardPosition,
    to: BoardPosition,
    current_turn: u32,
) -> Result<Option<Entity>, String> {
    // 移動元の駒を検索
    let moving_piece_entity = board.get_piece_at(from)
        .ok_or("No piece at source position")?;

    // 移動先に駒があるかチェック（キャプチャ）
    let captured_piece = board.get_piece_at(to);

    // 駒を移動
    if let Ok((_, mut piece, transform)) = pieces.get_mut(moving_piece_entity) {
        let start_pos = transform.translation;
        let target_pos = to.to_world_position();

        // アニメーションを開始
        commands.entity(moving_piece_entity).insert(
            PieceAnimation::new_move(start_pos, target_pos, time.elapsed_secs())
        );

        // 駒の論理的な位置を更新
        piece.set_position(to);
        piece.mark_moved(current_turn);

        // 盤面を更新
        board.set_piece_at(from, None);
        board.set_piece_at(to, Some(moving_piece_entity));

        // キャプチャされた駒にアニメーションを適用
        if let Some(captured_entity) = captured_piece {
            if let Ok((_, _, transform)) = pieces.get(captured_entity) {
                commands.entity(captured_entity).insert(
                    PieceAnimation::new_capture(transform.translation, time.elapsed_secs())
                );
            }
        }

        Ok(captured_piece)
    } else {
        Err("Invalid piece entity".to_string())
    }
}

pub fn update_piece_effects(
    mut pieces: Query<(&mut PieceEffect, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    for (mut effect, material_handle) in pieces.iter_mut() {
        if let Some(duration) = effect.duration {
            if current_time - effect.start_time > duration {
                // エフェクト終了 - 元のマテリアルに戻す処理が必要
                continue;
            }
        }

        if let Some(material) = materials.get_mut(material_handle.id()) {
            match &effect.effect_type {
                PieceEffectType::Glow { color } => {
                    material.emissive = *color * effect.intensity;
                }
                PieceEffectType::Pulse { color, frequency } => {
                    let pulse = (current_time * frequency * 2.0 * std::f32::consts::PI).sin();
                    let intensity = effect.intensity * (0.5 + 0.5 * pulse);
                    material.emissive = *color * intensity;
                }
                _ => {} // 他のエフェクトタイプの実装
            }
        }
    }
}
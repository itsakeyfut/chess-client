use bevy::prelude::*;
use crate::game::{board::ChessBoard, ChessPiece};

pub fn setup_chess_board(
    mut commands: Commands,
    meshes: Res<crate::graphics::ChessMeshes>,
    materials: Res<crate::graphics::ChessMaterials>,
    mut board: ResMut<ChessBoard>,
) {
    info!("Setting up chess board...");
    crate::graphics::spawn_board(&mut commands, &meshes, &materials, &mut board);
}

pub fn setup_initial_pieces(
    commands: Commands,
    board: ResMut<ChessBoard>,
    meshes: Res<crate::graphics::ChessMeshes>,
    materials: Res<crate::graphics::ChessMaterials>,
) {
    info!("Setting up initial pieces...");
    crate::game::pieces::spawn_initial_pieces(commands, board, meshes, materials);
}

pub fn setup_game_camera(
    mut camera_controller: ResMut<crate::core::CameraController>,
) {
    info!("Setting up game camera...");
    camera_controller.reset_to_default();
}

// TODO: 駒の移動処理を行うこと
pub fn handle_piece_movement() { todo!() }
// TODO: 手を検証すること
pub fn validate_moves() { todo!() }
// TODO: ゲーム状態を更新すること
pub fn update_game_state() { todo!() }
// TODO: ゲーム終了条件をチェックすること
pub fn check_game_end_conditions() { todo!() }
// TODO: 合法手を更新すること
pub fn update_legal_moves() { todo!() }

pub fn update_piece_positions(
    mut pieces: Query<(&mut Transform, &ChessPiece), Without<crate::game::pieces::PieceAnimation>>,
) {
    for (mut transform, piece) in pieces.iter_mut() {
        let target_position = piece.position.to_world_position();
        transform.translation = target_position;
    }
}

// TODO: 盤面のハイライトを更新すること
pub fn update_board_highlights() { todo!() }

pub fn animate_pieces(
    commands: Commands,
    pieces: Query<(Entity, &mut Transform, &crate::game::pieces::PieceAnimation)>,
    time: Res<Time>,
) {
    crate::game::pieces::animate_pieces(commands, pieces, time);
}

// TODO: ゲームUIを更新すること
pub fn update_game_ui() { todo!() }
// TODO: 手履歴の表示を更新すること
pub fn update_move_history_display() { todo!() }

pub fn cleanup_game_entities(
    mut commands: Commands,
    game_entities: Query<Entity, Or<(
        With<ChessPiece>,
        With<crate::graphics::BoardEntity>,
    )>>,
) {
    for entity in game_entities.iter() {
        commands.entity(entity).despawn();
    }
}

// TODO: ゲーム状態を保存すること
pub fn save_game_state() { todo!() }
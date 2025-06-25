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

pub fn handle_piece_movement() { todo!() }
pub fn validate_moves() { todo!() }
pub fn update_game_state() { todo!() }
pub fn check_game_end_conditions() { todo!() }
pub fn update_legal_moves() { todo!() }

pub fn update_piece_positions(
    mut pieces: Query<(&mut Transform, &ChessPiece), Without<crate::game::pieces::PieceAnimation>>,
) {
    for (mut transform, piece) in pieces.iter_mut() {
        let target_position = piece.position.to_world_position();
        transform.translation = target_position;
    }
}

pub fn update_board_highlights() { todo!() }
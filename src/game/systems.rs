use bevy::prelude::*;
use crate::game::board::ChessBoard;

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

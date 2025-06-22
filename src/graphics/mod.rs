pub mod materials;
pub mod meshes;
pub mod lighting;
pub mod effects;
pub mod animations;
pub mod procedural;

use bevy::prelude::*;
use crate::core::{GameState, CoreSet};

pub use materials::*;
pub use meshes::*;
pub use lighting::*;
pub use effects::*;
pub use animations::*;
pub use procedural::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<ChessMeshes>()
            .init_resource::<ChessMaterials>()

            // Startup
            .add_systems(Startup, (
                setup_graphics,
            ).chain())

            // Update
            .add_systems(Update, (
                animate_materials,
            ).in_set(CoreSet::Graphics))

            // System when main game started
            .add_systems(OnEnter(GameState::InGame), (
                setup_board_graphics,
                // setup_piece_graphics,
            ).chain())

            .add_systems(OnExit(GameState::InGame), (
                cleanup_game_graphics,
            ));
    }
}

fn setup_graphics(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chess_meshes: ResMut<ChessMeshes>,
    mut chess_materials: ResMut<ChessMaterials>,
) {
    info!("Setting up graphics...");

    chess_meshes.initialize(&mut meshes);
    chess_materials.initialize(&mut materials);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::VISIBLE,
        MainCamera,
        Name::new("Main Camera"),
    ));
}

fn setup_board_graphics(
    commands: Commands,
    board: ResMut<crate::game::board::ChessBoard>,
    meshes: Res<ChessMeshes>,
    materials: Res<ChessMaterials>,
) {
    info!("Setting up piece graphics...");

    crate::game::pieces::spawn_initial_pieces(commands, board, meshes, materials);
}

fn cleanup_game_graphics(
    mut commands: Commands,
    game_entities: Query<Entity, Or<(
        With<crate::game::pieces::ChessPiece>,
        With<BoardSquare>,
        With<BoardEntity>,
    )>>,
) {
    for entity in game_entities.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct BoardEntity;

#[derive(Component)]
pub struct BoardSquare {
    pub position: crate::game::board::BoardPosition,
    pub is_light: bool,
}
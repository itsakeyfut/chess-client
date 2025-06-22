
pub mod board;
pub mod pieces;
pub mod moves;
pub mod rules;
pub mod state;
pub mod systems;

use bevy::prelude::*;
use crate::core::{GameState, CoreSet};

pub use board::*;
pub use pieces::*;
pub use moves::*;
pub use rules::*;
pub use state::*;
pub use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // リソース初期化
            .init_resource::<ChessBoard>()
            .init_resource::<GameStateResource>()
            .init_resource::<MoveHistory>()
            .init_resource::<SelectionState>()
            
            // システム追加
            .add_systems(OnEnter(GameState::InGame), (
                setup_chess_board,
                setup_initial_pieces,
                setup_game_camera,
            ).chain())
            
            .add_systems(Update, (
                // 入力処理
                handle_piece_selection,
                handle_piece_movement,
                validate_moves,
                
                // ゲームロジック
                update_game_state,
                check_game_end_conditions,
                update_legal_moves,
                
                // 表示更新
                update_piece_positions,
                update_board_highlights,
                animate_pieces,
                
                // UI更新
                update_game_ui,
                update_move_history_display,
            ).in_set(CoreSet::Logic))
            
            .add_systems(OnExit(GameState::InGame), (
                cleanup_game_entities,
                save_game_state,
            ));
    }
}
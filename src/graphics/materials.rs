
use bevy::prelude::*;
use crate::core::constants::*;

#[derive(Resource)]
pub struct ChessMaterials {
    // Pieces
    pub white_piece: Handle<StandardMaterial>,
    pub black_piece: Handle<StandardMaterial>,
    pub white_piece_selected: Handle<StandardMaterial>,
    pub black_piece_selected: Handle<StandardMaterial>,
    
    // Board
    pub light_square: Handle<StandardMaterial>,
    pub dark_square: Handle<StandardMaterial>,
    pub board_frame: Handle<StandardMaterial>,
    
    // Highlight
    pub selected_square: Handle<StandardMaterial>,
    pub legal_move_square: Handle<StandardMaterial>,
    pub last_move_square: Handle<StandardMaterial>,
    pub check_square: Handle<StandardMaterial>,
    pub capture_square: Handle<StandardMaterial>,
    pub threat_square: Handle<StandardMaterial>,
    
    // Effect
    pub glow_material: Handle<StandardMaterial>,
    pub particle_material: Handle<StandardMaterial>,
    pub transparent_overlay: Handle<StandardMaterial>,
}
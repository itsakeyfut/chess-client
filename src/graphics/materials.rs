
use bevy::prelude::*;
use crate::core::constants::*;

#[derive(Resource)]
pub struct ChessMaterials {
    pub white_piece: Handle<StandardMaterial>,
    pub black_piece: Handle<StandardMaterial>,
    pub light_square: Handle<StandardMaterial>,
    pub dark_square: Handle<StandardMaterial>,
    pub selected: Handle<StandardMaterial>,
    pub legal_move: Handle<StandardMaterial>,
    pub last_move: Handle<StandardMaterial>,
    pub check: Handle<StandardMaterial>,
    pub capture: Handle<StandardMaterial>,
    pub threat: Handle<StandardMaterial>,
}

impl Default for ChessMaterials {
    fn default() -> Self {
        Self {
            white_piece: Handle::default(),
            black_piece: Handle::default(),
            light_square: Handle::default(),
            dark_square: Handle::default(),
            selected: Handle::default(),
            legal_move: Handle::default(),
            last_move: Handle::default(),
            check: Handle::default(),
            capture: Handle::default(),
            threat: Handle::default(),
        }
    }
}

impl ChessMaterials {
    pub fn initialize(&mut self, materials: &mut Assets<StandardMaterial>) {
        // Pieces
        self.white_piece = materials.add(StandardMaterial {
            base_color: WHITE_PIECE_COLOR,
            perceptual_roughness: 0.3,
            metallic: 0.1,
            reflectance: 0.8,
            ..default()
        });

        self.black_piece = materials.add(StandardMaterial {
            base_color: BLACK_PIECE_COLOR,
            perceptual_roughness: 0.3,
            metallic: 0.1,
            reflectance: 0.8,
            ..default()
        });

        // Board
        self.light_square = materials.add(StandardMaterial {
            base_color: LIGHT_SQUARE_COLOR,
            perceptual_roughness: 0.8,
            metallic: 0.0,
            ..default()
        });

        self.dark_square = materials.add(StandardMaterial {
            base_color: DARK_SQUARE_COLOR,
            perceptual_roughness: 0.8,
            metallic: 0.0,
            ..default()
        });

        // Highlight
        self.selected = materials.add(StandardMaterial {
            base_color: SELECTED_COLOR,
            emissive: scale_color_linear(SELECTED_COLOR, 0.3),
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

        self.legal_move = materials.add(StandardMaterial {
            base_color: LEGAL_MOVE_COLOR,
            emissive: scale_color_linear(LEGAL_MOVE_COLOR, 0.2),
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

        self.last_move = materials.add(StandardMaterial {
            base_color: LAST_MOVE_COLOR,
            emissive: scale_color_linear(LAST_MOVE_COLOR, 0.1),
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

        self.check = materials.add(StandardMaterial {
            base_color: CHECK_COLOR,
            emissive: scale_color_linear(CHECK_COLOR, 0.5),
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

        self.capture = materials.add(StandardMaterial {
            base_color: CAPTURE_COLOR,
            emissive: scale_color_linear(CAPTURE_COLOR, 0.3),
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

        self.threat = materials.add(StandardMaterial {
            base_color: THREAT_COLOR,
            emissive: scale_color_linear(THREAT_COLOR, 0.2),
            alpha_mode: AlphaMode::Blend,
            ..default()
        });
    }

    pub fn get_square_material(&self, is_light: bool) -> Handle<StandardMaterial> {
        if is_light {
            self.light_square.clone()
        } else {
            self.dark_square.clone()
        }
    }

    pub fn get_piece_material(&self, color: crate::game::pieces::PieceColor) -> Handle<StandardMaterial> {
        match color {
            crate::game::pieces::PieceColor::White => self.white_piece.clone(),
            crate::game::pieces::PieceColor::Black => self.black_piece.clone(),
        }
    }
}

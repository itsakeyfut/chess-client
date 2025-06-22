use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct EffectSettings {
    pub enable_particles: bool,
    pub enable_glow: bool,
    pub enable_shadows: bool,
    pub particle_density: f32,
}

#[derive(Component)]
pub struct HighlightEffect {
    pub highlight_type: HighlightType,
    pub intensity: f32,
    pub duration: Option<f32>,
    pub start_time: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum HighlightType {
    Selected,
    LegalMove,
    LastMove,
    Check,
    Capture,
    Threat,
}

impl HighlightType {
    pub fn get_color(self) -> Color {
        use crate::core::constants::*;
        match self {
            HighlightType::Selected => SELECTED_COLOR,
            HighlightType::LegalMove => LEGAL_MOVE_COLOR,
            HighlightType::LastMove => LAST_MOVE_COLOR,
            HighlightType::Check => CHECK_COLOR,
            HighlightType::Capture => CAPTURE_COLOR,
            HighlightType::Threat => THREAT_COLOR,
        }
    }
}

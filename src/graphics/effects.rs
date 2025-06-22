use bevy::prelude::*;

use crate::scale_color_linear;

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

pub fn update_effects(
    mut commands: Commands,
    mut highlighted: Query<(Entity, &mut MaterialHandle, &HighlightEffect)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    let curr_time = time.elapsed_secs();

    for (entity, material_handle, effect) in highlighted.iter_mut() {
        if let Some(duration) = effect.duration {
            if curr_time - effect.start_time > duration {
                commands.entity(entity).remove::<HighlightEffect>();
                continue;
            }
        }

        let intensity = if let Some(duration) = effect.duration {
            let progress = (curr_time - effect.start_time) / duration;
            effect.intensity * (1.0 - progress).max(0.0)
        } else {
            effect.intensity
        };

        let pulse = (curr_time * 3.0).sin() * 0.5 + 0.5;
        let final_intensity = intensity * (0.5 + pulse * 0.5);

        let material_handle = &material_handle.0;

        if let Some(material) = materials.get_mut(material_handle) {
            let base_color = effect.highlight_type.get_color();
            material.emissive = scale_color_linear(base_color, final_intensity);
        }
    }
}

#[derive(Component)]
pub struct MaterialHandle(pub Handle<StandardMaterial>);

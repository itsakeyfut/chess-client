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

fn get_highlight_material(
    highlight_type: HighlightType,
    materials: &crate::graphics::ChessMaterials,
) -> Handle<StandardMaterial> {
    match highlight_type {
        HighlightType::Selected => materials.selected.clone(),
        HighlightType::LegalMove => materials.legal_move.clone(),
        HighlightType::LastMove => materials.last_move.clone(),
        HighlightType::Check => materials.check.clone(),
        HighlightType::Capture => materials.capture.clone(),
        HighlightType::Threat => materials.threat.clone(),
    }
}

pub fn add_highlight_effect(
    commands: &mut Commands,
    entity: Entity,
    highlight_type: HighlightType,
    intensity: f32,
    duration: Option<f32>,
    current_time: f32,
) {
    commands.entity(entity).insert(HighlightEffect {
        highlight_type,
        intensity,
        duration,
        start_time: current_time,
    });
}

#[derive(Component)]
pub struct ParticleEffect {
    pub effect_type: ParticleType,
    pub spawn_rate: f32,
    pub lifetime: f32,
    pub start_time: f32,
}

#[derive(Debug, Clone)]
pub enum ParticleType {
    Sparkle,
    Smoke,
    Magic,
    Explosion,
}

pub fn handle_graphics_settings(
    settings: Res<crate::core::resources::GameSettings>,
    mut effect_settings: ResMut<EffectSettings>,
    mut lighting_settings: ResMut<crate::graphics::LightingSettings>,
) {
    if settings.is_changed() {
        match settings.graphics_quality {
            crate::core::resources::GraphicsQuality::Low => {
                effect_settings.enable_particles = false;
                effect_settings.enable_glow = false;
                effect_settings.particle_density = 0.3;
                lighting_settings.enable_shadows = false;
            },
            crate::core::resources::GraphicsQuality::Medium => {
                effect_settings.enable_particles = true;
                effect_settings.enable_glow = false;
                effect_settings.particle_density = 0.6;
                lighting_settings.enable_shadows = true;
                lighting_settings.shadow_quality = 0.5;
            },
            crate::core::resources::GraphicsQuality::High => {
                effect_settings.enable_particles = true;
                effect_settings.enable_glow = true;
                effect_settings.particle_density = 1.0;
                lighting_settings.enable_shadows = true;
                lighting_settings.shadow_quality = 1.0;
            },
            crate::core::resources::GraphicsQuality::Ultra => {
                effect_settings.enable_particles = true;
                effect_settings.enable_glow = true;
                effect_settings.particle_density = 1.5;
                lighting_settings.enable_shadows = true;
                lighting_settings.shadow_quality = 2.0;
            },
        }
    }
}

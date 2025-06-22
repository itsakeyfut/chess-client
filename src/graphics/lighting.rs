use bevy::prelude::*;
use crate::core::constants::*;

#[derive(Resource)]
pub struct LightingSettings {
    pub ambient_intensity: f32,
    pub directional_intensity: f32,
    pub directional_direction: Vec3,
    pub enable_shadows: bool,
    pub shadow_quality: f32,
}

impl Default for LightingSettings {
    fn default() -> Self {
        Self {
            ambient_intensity: AMBIENT_LIGHT_STRENGTH,
            directional_intensity: DIRECTIONAL_LIGHT_STRENGTH,
            directional_direction: Vec3::new(-0.5, -1.0, -0.5).normalize(),
            enable_shadows: true,
            shadow_quality: 1.0,
        }
    }
}

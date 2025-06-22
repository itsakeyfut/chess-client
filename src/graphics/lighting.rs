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

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct EffectSettings {
    pub enable_particles: bool,
    pub enable_glow: bool,
    pub enable_shadows: bool,
    pub particle_density: f32,
}
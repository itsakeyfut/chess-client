use bevy::prelude::*;

#[derive(Component)]
pub struct BoardAnimation {
    pub animation_type: BoardAnimationType,
    pub start_time: f32,
    pub duration: f32,
    pub progress: f32,
}

#[derive(Debug, Clone)]
pub enum BoardAnimationType {
    Flip { from_angle: f32, to_angle: f32 },
    Shake { intensity: f32 },
    Pulse { color: Color },
}

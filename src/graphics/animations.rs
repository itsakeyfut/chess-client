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

pub fn animate_board(
    mut board_entities: Query<(&mut Transform, &BoardAnimation), With<super::BoardEntity>>,
    time: Res<Time>,
) {
    let curr_time = time.elapsed_secs();

    for (mut transform, animation) in board_entities.iter_mut() {
        let progress = ((curr_time - animation.start_time) / animation.duration).clamp(0.0, 1.0);
        let eased_progress = crate::core::constants::ease_in_out_cubic(progress);

        match &animation.animation_type {
            BoardAnimationType::Flip { from_angle, to_angle } => {
                let curr_angle = from_angle + (to_angle - from_angle) * eased_progress;
                transform.rotation = Quat::from_rotation_y(curr_angle.to_radians());
            },
            BoardAnimationType::Shake { intensity } => {
                if progress < 1.0 {
                    let shake_amount = intensity * (1.0 - progress);
                    let shake_x = (curr_time * 20.0).sin() * shake_amount;
                    let shake_z = (curr_time * 25.0).cos() * shake_amount;
                    transform.translation.x += shake_x * 0.01;
                    transform.translation.z += shake_z * 0.01;
                }
            },
            BoardAnimationType::Pulse { .. } => {
                // パルスエフェクトは別のシステムで処理
            },
        }
    }
}

#[derive(Component)]
pub struct CameraAnimation {
    pub target_position: Vec3,
    pub target_look_at: Vec3,
    pub start_position: Vec3,
    pub start_look_at: Vec3,
    pub start_time: f32,
    pub duration: f32,
    pub ease_type: CameraEaseType,
}

#[derive(Debug, Clone)]
pub enum CameraEaseType {
    Linear,
    EaseInOut,
    EaseOut,
    Smooth,
}

pub fn animate_camera(
    mut cameras: Query<(Entity, &mut Transform, &CameraAnimation), With<super::MainCamera>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();
    
    for (entity, mut transform, animation) in cameras.iter_mut() {
        let progress = ((current_time - animation.start_time) / animation.duration).clamp(0.0, 1.0);
        
        let eased_progress = match animation.ease_type {
            CameraEaseType::Linear => progress,
            CameraEaseType::EaseInOut => crate::core::constants::ease_in_out_cubic(progress),
            CameraEaseType::EaseOut => 1.0 - (1.0 - progress).powi(3),
            CameraEaseType::Smooth => progress * progress * (3.0 - 2.0 * progress),
        };
        
        let current_position = animation.start_position.lerp(animation.target_position, eased_progress);
        let current_look_at = animation.start_look_at.lerp(animation.target_look_at, eased_progress);
        
        transform.translation = current_position;
        transform.look_at(current_look_at, Vec3::Y);
        
        if progress >= 1.0 {
            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.remove::<CameraAnimation>();
            }
        }
    }
}

pub fn start_camera_animation(
    commands: &mut Commands,
    camera_entity: Entity,
    target_position: Vec3,
    target_look_at: Vec3,
    current_transform: &Transform,
    duration: f32,
    ease_type: CameraEaseType,
    current_time: f32,
) {
    commands.entity(camera_entity).insert(CameraAnimation {
        target_position,
        target_look_at,
        start_position: current_transform.translation,
        start_look_at: current_transform.forward().into(),
        start_time: current_time,
        duration,
        ease_type,
    });
}

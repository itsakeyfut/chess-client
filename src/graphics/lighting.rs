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

pub fn setup_lighting(
    mut commands: Commands,
    lighting_settings: Res<LightingSettings>,
) {
    // Environment lighting
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: lighting_settings.ambient_intensity,
        affects_lightmapped_meshes: true,
    });

    // Sunshine
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: lighting_settings.directional_intensity * 10_000.0,
            shadows_enabled: lighting_settings.enable_shadows,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_x(DIRECTIONAL_LIGHT_ANGLE.to_radians()))
            .looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::VISIBLE,
        ViewVisibility::default(),
        Name::new("Directional Light"),
        MainLight,
    ));
    

    // Fill light
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.9, 0.9, 1.0),
            illuminance: lighting_settings.directional_intensity * 3000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_x((-30.0_f32).to_radians()))
            .with_rotation(Quat::from_rotation_y(45.0_f32.to_radians())),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::VISIBLE,
        ViewVisibility::default(),
        Name::new("Fill Light")
    ));

    // Rim light
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.9, 0.8),
            illuminance: lighting_settings.directional_intensity * 2000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_x(30.0_f32.to_radians()))
            .with_rotation(Quat::from_rotation_y((-135.0_f32).to_radians())),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::VISIBLE,
        ViewVisibility::default(),
        Name::new("Rim Light")
    ));
}

pub fn update_lighting(
    mut lights: Query<&mut DirectionalLight, With<MainLight>>,
    lighting_settings: Res<LightingSettings>,
    time: Res<Time>,
) {
    if lighting_settings.is_changed() {
        for mut light in lights.iter_mut() {
            light.illuminance = lighting_settings.directional_intensity * 10000.0;
            light.shadows_enabled = lighting_settings.enable_shadows;
        }
    }

    let flicker = (time.elapsed_secs() * 0.5).sin() * 0.02 + 1.0;
    for mut light in lights.iter_mut() {
        light.illuminance *= flicker;
    }
}

#[derive(Component)]
pub struct MainLight;

#[derive(Component)]
pub struct DynamicLight {
    pub base_intensity: f32,
    pub flicker_speed: f32,
    pub flicker_amount: f32,
}

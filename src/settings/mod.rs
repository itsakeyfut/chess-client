use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        // TODO: 設定システムの実装
        info!("Settings plugin loaded (placeholder)");
    }
}
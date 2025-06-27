use bevy::prelude::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        // TODO: ネットワークシステムの実装
        info!("Network plugin loaded (placeholder)");
    }
}
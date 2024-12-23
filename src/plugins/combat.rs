use crate::systems::combat::*;
use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_enemy)
            // .add_systems(Update, combat_debug)
            .add_systems(Update, handle_combat_collision);

        info!("Combat plugin initialized");
    }
}

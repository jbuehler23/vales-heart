use crate::{resources::GameState, systems::{class::spawn_selected_player, combat::*, input::mouse_aim_system}};
use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), spawn_test_enemy)
            .add_systems(Update, (
                mouse_aim_system,
                handle_combat_collisions,
                update_floating_text,
            ).run_if(in_state(GameState::Playing)));
    }
}

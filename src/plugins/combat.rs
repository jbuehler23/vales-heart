use crate::{resources::GameState, systems::{class::spawn_selected_player, combat::*}};
use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::Playing), 
                spawn_test_enemy.after(spawn_selected_player)
            );
    }
}

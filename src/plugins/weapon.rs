use bevy::prelude::*;
use crate::{resources::GameState, systems::weapon::*};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Startup, spawn_weapons)
            .add_systems(Update, (
                weapon_attack_system,
                projectile_system,
            ).run_if(in_state(GameState::Playing)));
    }
}
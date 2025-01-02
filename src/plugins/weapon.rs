use bevy::prelude::*;
use crate::{resources::GameState, systems::weapon::*};

pub struct WeaponPlugin;

// src/plugins/weapon.rs
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            weapon_attack_system,
            projectile_system,
            update_player_direction,
            update_melee_swing
        ).run_if(in_state(GameState::Playing)));
    }
}
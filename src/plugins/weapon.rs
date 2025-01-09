use bevy::prelude::*;
use crate::{resources::GameState, systems::weapon::*};

pub struct WeaponPlugin;

// src/plugins/weapon.rs
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_weapon_input,
            update_player_direction,
        ).run_if(in_state(GameState::Playing)));
    }
}

// Add these events to your plugin registration
#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: f32,
    pub source: Entity,
}

#[derive(Event)]
pub struct HitEffectEvent {
    pub position: Vec3,
    pub damage: f32,
}
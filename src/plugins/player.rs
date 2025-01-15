use crate::{components::{inventory::Equipment, player::*}, resources::GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, CoefficientCombineRule, Collider, Friction, GravityScale, LockedAxes, RigidBody, Velocity};
// use crate::components::stats::*;
use crate::systems::{input::player_input, movement::player_movement};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            player_input,
            player_movement
        ).run_if(in_state(GameState::Playing)));
    }
}
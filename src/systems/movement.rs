use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use crate::components::player::{Player, MovementInput};

// Update our movement system to work with physics
pub fn player_movement(
    time: Res<Time>,
    mut query: Query<(&Player, &MovementInput, &mut Velocity)>,
) {
    for (player, input, mut velocity) in query.iter_mut() {
        let movement = Vec2::new(
            input.x * player.speed,
            input.y * player.speed,
        );
        velocity.linvel = movement;
    }
}

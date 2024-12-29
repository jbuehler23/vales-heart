use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::components::player::{MovementInput, Player};

// Add movement system
pub fn player_movement(
    time: Res<Time>,
    mut query: Query<(&Player, &MovementInput, &mut Transform)>,
) {
    for (player, input, mut transform) in query.iter_mut() {
        let movement = Vec2::new(input.x, input.y);
        if movement != Vec2::ZERO {
            let movement = movement.normalize() * player.speed * time.delta_secs();
            transform.translation.x += movement.x;
            transform.translation.y += movement.y;
        }
    }
}
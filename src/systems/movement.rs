use bevy::prelude::*;
use crate::components::player::{Player, MovementInput};

pub fn player_movement(
    time: Res<Time>,
    mut query: Query<(&Player, &MovementInput, &mut Transform)>,
) {
    for (player, input, mut transform) in query.iter_mut() {
        let movement = Vec3::new(
            input.x * player.speed * time.delta_secs(),
            input.y * player.speed * time.delta_secs(),
            0.0,
        );
        transform.translation += movement;
    }
}

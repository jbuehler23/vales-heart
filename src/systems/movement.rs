use bevy::prelude::*;

use crate::components::player::{Direction, MovementInput, Player};

// Add movement system
pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        let mut movement = Vec2::ZERO;
        let mut new_direction = player.direction;

        // WASD movement
        if keyboard.pressed(KeyCode::KeyW) {
            movement.y += 1.0;
            new_direction = Direction::Up;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            movement.y -= 1.0;
            new_direction = Direction::Down;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
            new_direction = Direction::Left;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            movement.x += 1.0;
            new_direction = Direction::Right;
        }

        // Update player state
        player.is_moving = movement != Vec2::ZERO;
        if player.is_moving {
            player.direction = new_direction;
        }

        // Apply movement
        if movement != Vec2::ZERO {
            movement = movement.normalize();
            transform.translation += Vec3::new(
                movement.x * player.speed * time.delta_secs(),
                movement.y * player.speed * time.delta_secs(),
                0.0,
            );
        }
    }
}
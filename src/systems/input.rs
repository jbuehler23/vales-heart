use bevy::prelude::*;
use crate::components::player::MovementInput;

pub fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MovementInput>,
) {
    for mut movement in query.iter_mut() {
        // Reset movement
        movement.x = 0.0;
        movement.y = 0.0;

        // Horizontal movement
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            movement.x += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            movement.x -= 1.0;
        }

        // Vertical movement
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            movement.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            movement.y -= 1.0;
        }

        // Normalize diagonal movement
        if movement.x != 0.0 && movement.y != 0.0 {
            let movement_vec = Vec2::new(movement.x, movement.y).normalize();
            movement.x = movement_vec.x;
            movement.y = movement_vec.y;
        }
    }
}

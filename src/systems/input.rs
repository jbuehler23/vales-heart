use bevy::prelude::*;
use crate::components::player::MovementInput;

pub fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MovementInput>,
) {
    for mut movement in query.iter_mut() {
        movement.x = if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            -1.0
        } else if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            1.0
        } else {
            0.0
        };

        movement.y = if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            1.0
        } else if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            -1.0
        } else {
            0.0
        };
    }
}

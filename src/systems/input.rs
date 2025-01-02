use bevy::prelude::*;
use crate::{components::player::{MovementInput, Player}, resources::GameState};

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

pub fn handle_escape_menu(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::Playing),
            _ => {}
        }
    }
}

pub fn mouse_aim_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = windows.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        for mut transform in player_query.iter_mut() {
            let direction = (world_position - transform.translation.truncate()).normalize();
            transform.rotation = Quat::from_rotation_z(direction.y.atan2(direction.x));
        }
    }
}

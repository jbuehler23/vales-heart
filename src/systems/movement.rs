use bevy::prelude::*;

use crate::components::player::{MovementInput, Player};

// Add movement system
pub fn player_movement(
    time: Res<Time>,
    mut query: Query<(&Player, &MovementInput, &mut Transform)>,
) {
    for (player, movement, mut transform) in query.iter_mut() {
        let movement_vector = Vec2::new(movement.x, movement.y);
        
        // Only move if there's input
        if movement_vector != Vec2::ZERO {
            // Normalize for consistent speed in all directions
            let movement_vector = movement_vector.normalize();
            
            // Calculate frame-independent movement
            let movement_delta = movement_vector * player.speed * time.delta_secs();
            
            // Update position
            transform.translation.x += movement_delta.x;
            transform.translation.y += movement_delta.y;
        }
    }
}
use bevy::prelude::*;

use crate::components::{animation::SpriteAnimation, player::MovementInput};

pub fn animation_system(
    time: Res<Time>,
    mut query: Query<(
        &mut SpriteAnimation,
        &mut Sprite,
        &MovementInput
    )>,
) {
    for (mut animation, mut sprite, movement) in query.iter_mut() {
        // Update movement state
        animation.is_moving = movement.x != 0.0 || movement.y != 0.0;
        
        // Update facing direction
        if movement.x != 0.0 {
            animation.facing_left = movement.x < 0.0;
        }
        
        // Update frame if moving
        if animation.is_moving {
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
            }
        }

        // Update sprite
        sprite.flip_x = animation.facing_left;
    }
}
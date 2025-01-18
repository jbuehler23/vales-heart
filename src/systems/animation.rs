use bevy::prelude::*;

use crate::components::{animation::SpriteAnimation, player::{Direction, MovementInput, Player}};

pub fn update_walk_animation(
    time: Res<Time>,
    mut query: Query<(
        &mut SpriteAnimation,
        &mut Sprite,
        &Player,
        &MovementInput
    )>,
) {
    for (mut anim, mut sprite, player, movement) in query.iter_mut() {
        let is_moving = movement.x != 0.0 || movement.y != 0.0;

        // Update animation frame if moving
        if is_moving {
            anim.timer.tick(time.delta());
            if anim.timer.just_finished() {
                anim.current_frame = (anim.current_frame + 1) % anim.frames.len();
                if let Some(texture) = anim.frames.get(anim.current_frame) {
                    sprite.image = texture.clone();
                }
            }
        } else {
            // Reset to first frame when idle
            if let Some(texture) = anim.frames.first() {
                sprite.image = texture.clone();
            }
            anim.current_frame = 0;
        }

        // Handle sprite flipping
        sprite.flip_x = matches!(player.direction, Direction::Left);
    }
}
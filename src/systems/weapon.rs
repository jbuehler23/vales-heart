use bevy::prelude::*;
use crate::components::{
    player::{Direction, MovementInput, Player}, weapon::{Projectile, Weapon, WeaponItem}
};

pub fn handle_weapon_input(
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(Entity, &Transform, &Player), With<Player>>,
    mut weapon_query: Query<&mut WeaponItem>,
) {
    // Only process if space is pressed
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    if let Ok((player_entity, player_transform, player)) = player_query.get_single() {
        if let Ok(mut weapon) = weapon_query.get_single_mut() {
            if !weapon.can_attack(time.elapsed_secs()) {
                return;
            }

            // Get attack direction based on player's current facing direction
            let direction = match player.direction {
                Direction::Up => Vec2::new(0.0, 1.0),
                Direction::Down => Vec2::new(0.0, -1.0),
                Direction::Left => Vec2::new(-1.0, 0.0),
                Direction::Right => Vec2::new(1.0, 0.0),
            };

            // Calculate spawn position in front of player
            let spawn_pos = player_transform.translation + Vec3::new(direction.x * 32.0, direction.y * 32.0, 0.0);

            // Update weapon state
            weapon.set_last_attack(time.elapsed_secs());

            // Spawn the attack
            weapon.spawn_attack(
                &mut commands,
                player_entity,
                &Transform {
                    translation: spawn_pos,
                    rotation: Quat::from_rotation_z(direction.y.atan2(direction.x)),
                    ..default()
                }
            );
        }
    }
}

pub fn update_player_direction(
    mut player_query: Query<(&mut Player, &MovementInput)>,
) {
    for (mut player, movement) in player_query.iter_mut() {
        if movement.x != 0.0 || movement.y != 0.0 {
            // Set direction based on WASD movement
            player.direction = if movement.y.abs() > movement.x.abs() {
                if movement.y > 0.0 {
                    Direction::Up
                } else {
                    Direction::Down
                }
            } else {
                if movement.x > 0.0 {
                    Direction::Right
                } else {
                    Direction::Left
                }
            };
        }
    }
}

fn get_facing_direction(facing: Direction) -> Vec2 {
    match facing {
        Direction::Up => Vec2::new(0.0, 1.0),
        Direction::Down => Vec2::new(0.0, -1.0),
        Direction::Left => Vec2::new(-1.0, 0.0),
        Direction::Right => Vec2::new(1.0, 0.0),
    }
}

pub fn projectile_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(Entity, &mut Projectile)>,
) {
    for (entity, mut projectile) in projectiles.iter_mut() {
        projectile.lifetime.tick(time.delta());
        if projectile.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
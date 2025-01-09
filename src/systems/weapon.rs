use bevy::{color::palettes::css::{BROWN, ORANGE_RED, RED}, prelude::*};
use bevy_rapier2d::prelude::*;
use crate::{components::{
    class::{ClassType, PlayerClass}, combat::{DamageNumber, Enemy, Health}, player::{Direction, MovementInput, Player}, weapon::{Weapon, WeaponItem, WeaponType}
}, plugins::weapon::{DamageEvent, HitEffectEvent}};

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub lifetime: Timer,
}

#[derive(Component)]
pub enum ProjectileType {
    Arrow,
    Fireball,
}

pub fn handle_weapon_input(
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut weapon_query: Query<&mut WeaponItem>,
) {
    // Only process if space is pressed
    if !keyboard.pressed(KeyCode::Space) {
        return;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = windows.single();

    // Get mouse position in world coordinates
    if let (Some(cursor_pos), Ok((player_entity, player_transform))) = (
        window.cursor_position().and_then(|cursor| {
            camera.viewport_to_world_2d(camera_transform, cursor).ok()
        }),
        player_query.get_single()
    ) {
        if let Ok(mut weapon) = weapon_query.get_single_mut() {
            if !weapon.can_attack(time.elapsed_secs()) {
                return;
            }

            // Calculate attack direction and position
            let direction = (cursor_pos - player_transform.translation.truncate()).normalize();
            let spawn_pos = player_transform.translation + Vec3::new(direction.x * 32.0, direction.y * 32.0, 0.0);

            // Update weapon state
            weapon.set_last_attack(time.elapsed_secs());

            // Spawn the attack
            weapon.spawn_attack(
                &mut commands,
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
            player.facing = if movement.x.abs() > movement.y.abs() {
                if movement.x > 0.0 { Direction::Right } else { Direction::Left }
            } else {
                if movement.y > 0.0 { Direction::Up } else { Direction::Down }
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
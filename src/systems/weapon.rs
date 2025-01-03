use bevy::{color::palettes::css::{BROWN, ORANGE_RED, RED}, prelude::*};
use bevy_rapier2d::prelude::*;
use crate::components::{
    class::{ClassType, PlayerClass}, combat::{DamageNumber, Enemy, Health}, player::{Direction, MovementInput, Player}, weapon::{WeaponItem, WeaponType}
};

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

pub fn weapon_attack_system(
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    player_query: Query<(&Transform, &PlayerClass), With<Player>>,
    mut weapon_query: Query<&mut WeaponItem>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        if let Ok(world_position) = camera.viewport_to_world_2d(
            camera_transform,
            cursor_position,
        ) {
            for (player_transform, player_class) in player_query.iter() {
                if let Ok(mut weapon) = weapon_query.get_single_mut() {
                    if time.elapsed_secs() - weapon.last_attack < weapon.attack_speed {
                        continue;
                    }

                    weapon.last_attack = time.elapsed_secs();

                    // Calculate direction to mouse
                    let direction = (world_position - player_transform.translation.truncate()).normalize();
                    let spawn_position = player_transform.translation + Vec3::new(direction.x * 32.0, direction.y * 32.0, 0.0);

                    match player_class.class_type {
                        ClassType::Warrior => {
                            handle_melee_attack(&mut commands, spawn_position, direction, &weapon);
                        }
                        ClassType::Archer => {
                            spawn_projectile(
                                &mut commands,
                                ProjectileType::Arrow,
                                weapon.damage,
                                spawn_position,
                                direction,
                                400.0,
                            );
                        }
                        ClassType::Mage => {
                            spawn_projectile(
                                &mut commands,
                                ProjectileType::Fireball,
                                weapon.damage * 1.5,
                                spawn_position,
                                direction,
                                300.0,
                            );
                        }
                    }
                }
            }
        }
    }
}

fn handle_melee_attack(
    commands: &mut Commands,
    spawn_position: Vec3,
    direction: Vec2,
    weapon: &WeaponItem,
) {
    // Spawn swing arc entity
    let swing_time = 0.2; // Swing duration in seconds
    let swing_radius = 32.0; // Distance from player
    let swing_width = 48.0; // Width of swing arc
    let swing_height = 16.0; // Height of swing hitbox

    // Calculate initial rotation based on direction
    let initial_rotation = match (direction.x, direction.y) {
        (1.0, 0.0) => 0.0, // Right
        (-1.0, 0.0) => std::f32::consts::PI, // Left
        (0.0, 1.0) => -std::f32::consts::FRAC_PI_2, // Up
        (0.0, -1.0) => std::f32::consts::FRAC_PI_2, // Down
        _ => 0.0,
    };

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.9, 0.9, 0.9, 0.5),
                custom_size: Some(Vec2::new(swing_width, swing_height)),
                ..default()
            },
            transform: Transform {
                translation: spawn_position,
                rotation: Quat::from_rotation_z(initial_rotation),
                ..default()
            },
            ..default()
        },
        MeleeSwing {
            damage: weapon.damage,
            lifetime: Timer::from_seconds(swing_time, TimerMode::Once),
            start_angle: initial_rotation,
            end_angle: initial_rotation + std::f32::consts::PI * 0.75, // 135-degree swing
        },
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(swing_width / 2.0, swing_height / 2.0),
        Sensor,
        ActiveEvents::COLLISION_EVENTS,
    ));
}

#[derive(Component)]
pub struct MeleeSwing {
    pub damage: f32,
    pub lifetime: Timer,
    pub start_angle: f32,
    pub end_angle: f32,
}

// Add to your Update systems:
pub fn update_melee_swing(
    mut commands: Commands,
    time: Res<Time>,
    mut swing_query: Query<(Entity, &mut MeleeSwing, &mut Transform)>,
) {
    for (entity, mut swing, mut transform) in swing_query.iter_mut() {
        swing.lifetime.tick(time.delta());
        
        if swing.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // Calculate current angle based on lifetime progress
        let progress = swing.lifetime.fraction();
        let current_angle = swing.start_angle + (swing.end_angle - swing.start_angle) * progress;
        
        // Update transform rotation
        transform.rotation = Quat::from_rotation_z(current_angle);
    }
}

fn spawn_projectile(
    commands: &mut Commands,
    projectile_type: ProjectileType,
    damage: f32,
    position: Vec3,
    direction: Vec2,
    speed: f32,
) {
    let (color, size) = match projectile_type {
        ProjectileType::Arrow => (Color::from(BROWN), Vec2::new(16.0, 4.0)),
        ProjectileType::Fireball => (Color::from(ORANGE_RED), Vec2::new(12.0, 12.0)),
    };

    // let rotation = if direction.x != 0.0 {
    //     Quat::from_rotation_z(0.0)
    // } else {
    //     Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)
    // };

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform {
                translation: position,
                rotation: Quat::from_rotation_z(direction.y.atan2(direction.x)),
                ..default()
            },
            ..default()
        },
        projectile_type,
        Projectile {
            damage,
            speed,
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
        },
        RigidBody::Dynamic,
        Collider::cuboid(size.x / 2.0, size.y / 2.0),
        Velocity::linear(direction * speed),
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
        ActiveEvents::COLLISION_EVENTS,
    ));
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
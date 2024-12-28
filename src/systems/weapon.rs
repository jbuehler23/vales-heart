// src/systems/weapon.rs
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::{
    class::{ClassType, PlayerClass}, combat::{Enemy, Health}, player::Player, weapon::{Projectile, Weapon, WeaponType}
};

pub fn spawn_weapon_for_player(
    mut commands: Commands,
    player_query: Query<(Entity, &PlayerClass), Added<Player>>,
) {
    for (player_entity, player_class) in player_query.iter() {
        let weapon_config = match player_class.class_type {
            ClassType::Warrior => (
                WeaponType::Melee,
                Color::from(Srgba::GREEN),
                15.0,  // damage
                32.0,  // range
                1.0,   // attack speed
                Vec2::new(16.0, 0.0), // offset
            ),
            ClassType::Archer => (
                WeaponType::Ranged,
                Color::from(Srgba::BLUE),
                10.0,  // damage
                150.0, // range
                0.8,   // attack speed
                Vec2::new(0.0, 8.0),  // offset
            ),
            ClassType::Mage => (
                WeaponType::Ranged,
                Color::from(Srgba::BLACK),
                8.0,   // damage
                100.0, // range
                1.2,   // attack speed
                Vec2::new(0.0, 8.0),  // offset
            ),
        };

        info!("Spawning {:?} weapon for player", player_class.class_type);
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: weapon_config.1,
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    weapon_config.5.x,
                    weapon_config.5.y,
                    0.0
                ),
                ..default()
            },
            Weapon {
                weapon_type: weapon_config.0,
                damage: weapon_config.2,
                attack_range: weapon_config.3,
                attack_speed: weapon_config.4,
                last_attack: 0.0,
            },
            RigidBody::Fixed,
            Collider::cuboid(8.0, 8.0),
            ActiveEvents::COLLISION_EVENTS,
        )).set_parent(player_entity);
    }
}

pub fn weapon_attack_system(
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut weapon_query: Query<(&mut Weapon, &Transform, &Parent)>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    for (mut weapon, weapon_transform, parent) in weapon_query.iter_mut() {
        if time.elapsed_secs() - weapon.last_attack < weapon.attack_speed {
            continue;
        }

        if let Ok(player_transform) = player_query.get(parent.get()) {
            weapon.last_attack = time.elapsed_secs();

            match weapon.weapon_type {
                WeaponType::Melee => {
                    for (enemy_entity, enemy_transform, mut health) in enemy_query.iter_mut() {
                        let distance = enemy_transform.translation.distance(player_transform.translation);
                        if distance <= weapon.attack_range {
                            info!("Melee hit on enemy! Distance: {}", distance);
                            health.current -= weapon.damage;
                            
                            if health.current <= 0.0 {
                                commands.entity(enemy_entity).despawn();
                            }
                        }
                    }
                }
                WeaponType::Ranged => spawn_projectile(&mut commands, weapon.damage, player_transform),
            }
        }
    }
}

fn spawn_projectile(
    commands: &mut Commands,
    damage: f32,
    player_transform: &Transform,
) {
    let direction = Vec2::new(1.0, 0.0); // TODO: Use actual aim direction
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..default()
            },
            transform: Transform::from_translation(player_transform.translation),
            ..default()
        },
        Projectile {
            damage,
            speed: 200.0,
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
        },
        RigidBody::Dynamic,
        Velocity::linear(direction * 200.0),
        Collider::ball(4.0),
        ActiveEvents::COLLISION_EVENTS,
    ));
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

pub fn handle_weapon_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<(Entity, &Projectile)>,
    mut enemy_query: Query<(Entity, &mut Health), With<Enemy>>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            if let Some((projectile_entity, enemy_entity)) = find_projectile_enemy_collision(
                *entity1,
                *entity2,
                &projectile_query,
                &enemy_query,
            ) {
                if let Ok((_, projectile)) = projectile_query.get(projectile_entity) {
                    if let Ok((_, mut health)) = enemy_query.get_mut(enemy_entity) {
                        health.current -= projectile.damage;
                        commands.entity(projectile_entity).despawn();
                        
                        if health.current <= 0.0 {
                            commands.entity(enemy_entity).despawn();
                        }
                    }
                }
            }
        }
    }
}

fn find_projectile_enemy_collision(
    entity1: Entity,
    entity2: Entity,
    projectile_query: &Query<(Entity, &Projectile)>,
    enemy_query: &Query<(Entity, &mut Health), With<Enemy>>,
) -> Option<(Entity, Entity)> {
    let is_projectile1 = projectile_query.contains(entity1);
    let is_projectile2 = projectile_query.contains(entity2);
    let is_enemy1 = enemy_query.contains(entity1);
    let is_enemy2 = enemy_query.contains(entity2);

    match (is_projectile1, is_enemy2, is_projectile2, is_enemy1) {
        (true, true, _, _) => Some((entity1, entity2)),
        (_, _, true, true) => Some((entity2, entity1)),
        _ => None,
    }
}
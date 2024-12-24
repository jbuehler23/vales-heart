use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::{
    combat::{Enemy, Health}, player::Player, weapon::{Attack, Projectile, Weapon, WeaponType}
};

pub fn spawn_weapons(mut commands: Commands) {
    // Spawn melee weapon
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        Transform::from_xyz(50.0, 0.0, 1.0),
        Weapon {
            weapon_type: WeaponType::Melee,
            damage: 15.0,
            attack_range: 32.0,
            attack_speed: 1.0,
            last_attack: 0.0,
        },
        RigidBody::Fixed,
        Collider::cuboid(8.0, 8.0),
        ActiveEvents::COLLISION_EVENTS,
    ));

    // Spawn ranged weapon
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        Transform::from_xyz(50.0, 0.0, 1.0),
        Weapon {
            weapon_type: WeaponType::Ranged,
            damage: 10.0,
            attack_range: 150.0,
            attack_speed: 0.8,
            last_attack: 0.0,
        },
        RigidBody::Fixed,
        Collider::cuboid(8.0, 8.0),
        ActiveEvents::COLLISION_EVENTS,
    ));
}

pub fn weapon_attack_system(
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut weapon_query: Query<(&mut Weapon, &Transform)>,
    player_query: Query<(&Transform, &Player)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let Ok((player_transform, _)) = player_query.get_single() else { return };

    for (mut weapon, weapon_transform) in weapon_query.iter_mut() {
        if time.elapsed_secs() - weapon.last_attack < weapon.attack_speed {
            continue;
        }

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
            WeaponType::Ranged => {
                let direction = Vec2::new(1.0, 0.0); // Replace with actual aim direction
                commands.spawn((

                    Sprite {
                        color: Color::linear_rgb(1.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(8.0, 8.0)),
                        ..default()
                    },
                    Transform::from_translation(player_transform.translation),
                    Projectile {
                        damage: weapon.damage,
                        speed: 200.0,
                        lifetime: Timer::from_seconds(2.0, TimerMode::Once),
                    },
                    RigidBody::Dynamic,
                    Velocity::linear(direction * 200.0),
                    Collider::ball(4.0),
                    ActiveEvents::COLLISION_EVENTS,
                ));
            }
        }
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
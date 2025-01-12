use crate::{components::{combat::*, player::Player, weapon::ProjectileType}, utils::drops::drop_item};
use bevy::{
    color::palettes::css::{RED, WHITE},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

use super::weapon::Projectile;

pub fn spawn_test_enemy(mut commands: Commands) {
    info!("Spawning test enemy");

    commands.spawn((
        // Mesh2d(meshes.add(Circle::new(32.0))),
        // MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        Sprite {
            color: Color::linear_rgb(0.75, 0.0, 0.0),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(100.0, 100.0, 1.0),
        Enemy {
            damage: 10.0,
            detection_range: 100.0,
        },
        Health {
            current: 100.0,
            maximum: 100.0,
        },
        RigidBody::Fixed,
        Collider::cuboid(16.0, 16.0),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(0.0),
        Velocity::zero(),
        ColliderDebugColor(Color::WHITE.into()),
        CombatDebug,
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        ActiveEvents::COLLISION_EVENTS,
    ));
}

pub fn combat_debug(query: Query<(&Transform, &Health), With<CombatDebug>>) {
    for (transform, health) in query.iter() {
        info!(
            "Enemy at {:?} - Health: {}/{}",
            transform.translation, health.current, health.maximum
        );
    }
}

fn get_player_enemy_pair(
    entity1: Entity,
    entity2: Entity,
    player_query: &Query<Entity, With<Player>>,
    enemy_query: &Query<(Entity, &Enemy, &mut Health)>,
) -> Option<(Entity, Entity)> {
    let is_player1 = player_query.contains(entity1);
    let is_enemy1 = enemy_query.contains(entity1);
    let is_player2 = player_query.contains(entity2);
    let is_enemy2 = enemy_query.contains(entity2);

    match (is_player1, is_enemy1, is_player2, is_enemy2) {
        (true, false, false, true) => Some((entity1, entity2)),
        (false, true, true, false) => Some((entity2, entity1)),
        _ => None,
    }
}

// pub fn spawn_hit_effect(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut collision_events: EventReader<CollisionEvent>,
//     projectile_query: Query<(&Transform, &Projectile)>,
//     enemy_query: Query<&Transform, With<Enemy>>,
// ) {
//     for collision_event in collision_events.read() {
//         if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
//             if let Ok((proj_transform, projectile)) = projectile_query.get(*entity1) {
//                 if let Ok(enemy_transform) = enemy_query.get(*entity2) {
//                     spawn_damage_number(&mut commands, projectile.damage, enemy_transform.translation);
//                 }
//             }
//         }
//     }
// }

fn spawn_floating_text(
    commands: &mut Commands,
    position: Vec3,
    damage: f32,
    color: Color,
    parent: Entity,
) {
    let angle = rand::random::<f32>() * std::f32::consts::PI; // Random angle between 0 and PI
    let speed = 100.0 + rand::random::<f32>() * 50.0; // Random initial speed

    commands.spawn((
        Text2d::new(damage.to_string()),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        Transform::from_translation(position + Vec3::new(0.0, 20.0, 10.0)),
        TextColor(color),
        FloatingText {
            lifetime: Timer::from_seconds(0.8, TimerMode::Once),
            velocity: Vec2::new(
                angle.cos() * speed, // X velocity
                angle.sin() * speed, // Y velocity
            ),
            fade: 1.0,
            initial_y_offset: 40.0,
            gravity: -250.0, // Downward gravity
            parent,
        },
    ));
}

pub fn update_floating_text(
    mut commands: Commands,
    time: Res<Time>,
    mut param_set: ParamSet<(
        Query<(Entity, &mut FloatingText, &mut Transform, &mut TextColor)>,
        Query<&Transform, With<Enemy>>,
    )>,
) {
    // First, collect parent positions
    let mut parent_positions: Vec<(Entity, Vec3)> = Vec::new();
    let floating_texts: Vec<_> = param_set.p0().iter().map(|(e, ft, _, _)| (e, ft.parent)).collect();
    let enemy_query = param_set.p1();
    
    for (entity, parent) in floating_texts {
        if let Ok(parent_transform) = enemy_query.get(parent) {
            parent_positions.push((entity, parent_transform.translation));
        }
    }

    // Then update floating texts
    let mut text_query = param_set.p0();
    for (entity, mut floating_text, mut transform, mut text_color) in text_query.iter_mut() {
        floating_text.lifetime.tick(time.delta());
        let dt = time.delta_secs();

        // Find parent position
        if let Some(parent_pos) = parent_positions
            .iter()
            .find(|(e, _)| *e == entity)
            .map(|(_, pos)| *pos)
        {
            // Calculate offset from parent
            let offset = Vec3::new(
                floating_text.velocity.x * dt,
                floating_text.velocity.y * dt + floating_text.initial_y_offset,
                10.0,
            );

            // Update velocity for next frame
            floating_text.velocity.y += floating_text.gravity * dt;

            // Set position relative to parent
            transform.translation = parent_pos + offset;

            // Fade out
            floating_text.fade = 1.0 - (floating_text.lifetime.fraction() * floating_text.lifetime.fraction());
            text_color.0 = text_color.0.with_alpha(floating_text.fade);
        }

        if floating_text.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}


pub fn handle_weapon_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    projectiles: Query<(Entity, &Projectile, Option<&ProjectileType>)>,
    mut enemies: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            let (projectile_entity, enemy_entity) =
                if projectiles.contains(*entity1) && enemies.contains(*entity2) {
                    (*entity1, *entity2)
                } else if projectiles.contains(*entity2) && enemies.contains(*entity1) {
                    (*entity2, *entity1)
                } else {
                    continue;
                };

            if let Ok((_, projectile, projectile_type)) = projectiles.get(projectile_entity) {
                if let Ok((_, transform, mut health)) = enemies.get_mut(enemy_entity) {
                    let damage = if let Some(ProjectileType::Fireball) = projectile_type {
                        projectile.damage * 1.2
                    } else {
                        projectile.damage
                    };

                    health.current -= damage;
                    commands.entity(projectile_entity).despawn();

                    // Spawn damage number
                    spawn_floating_text(
                        &mut commands,
                        transform.translation + Vec3::new(0.0, 20.0, 0.0),
                        damage,
                        Color::from(WHITE),
                        enemy_entity,
                    );

                    if health.current <= 0.0 {
                        commands.entity(enemy_entity).despawn();
                        drop_item(0.5, transform, &mut commands);
                    }
                }
            }
        }
    }
}

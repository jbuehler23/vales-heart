use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::{combat::*, player::Player};

pub fn spawn_test_enemy(mut commands: Commands) {
    info!("Spawning test enemy");
    
    commands.spawn((
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
        RigidBody::Dynamic,
        Collider::cuboid(16.0, 16.0),
        ColliderDebugColor(Color::WHITE.into()),
        CombatDebug,
    ));
}

pub fn combat_debug(
    query: Query<(&Transform, &Health), With<CombatDebug>>,
) {
    for (transform, health) in query.iter() {
        info!(
            "Enemy at {:?} - Health: {}/{}",
            transform.translation, health.current, health.maximum
        );
    }
}

pub fn handle_combat_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    mut enemy_query: Query<(Entity, &Enemy, &mut Health)>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                // Check if one entity is the player and the other is an enemy
                if let Some((player_entity, enemy_entity)) = get_player_enemy_pair(
                    *entity1, 
                    *entity2,
                    &player_query,
                    &enemy_query
                ) {
                    if let Ok((_, enemy, mut health)) = enemy_query.get_mut(enemy_entity) {
                        info!("Combat collision between player and enemy!");
                        health.current -= enemy.damage;

                        if health.current <= 0.0 {
                            info!("Enemy defeated!");
                            commands.entity(enemy_entity).despawn();
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
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
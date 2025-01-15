use crate::components::{combat::*, player::Player, weapon::{Attack, Projectile, ProjectileType}};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::weapon::Weapon;


// Define collision groups
const PLAYER_GROUP: Group = Group::GROUP_1;
const ENEMY_GROUP: Group = Group::GROUP_2;
const ATTACK_GROUP: Group = Group::GROUP_3;


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
        CollisionGroups::new(ENEMY_GROUP,ATTACK_GROUP),
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

fn get_attack_pair(
    entity1: Entity,
    entity2: Entity,
    attacks: &Query<(Entity, &Attack)>,
    enemies: &Query<(Entity, &Transform, &mut Health), With<Enemy>>,
) -> Option<(Entity, Entity)> {
    if attacks.contains(entity1) && enemies.contains(entity2) {
        Some((entity1, entity2))
    } else if attacks.contains(entity2) && enemies.contains(entity1) {
        Some((entity2, entity1))
    } else {
        None
    }
}

fn get_projectile_pair(
    entity1: Entity,
    entity2: Entity,
    projectiles: &Query<(Entity, &Projectile, Option<&ProjectileType>)>,
    enemies: &Query<(Entity, &Transform, &mut Health), With<Enemy>>,
) -> Option<(Entity, Entity)> {
    if projectiles.contains(entity1) && enemies.contains(entity2) {
        Some((entity1, entity2))
    } else if projectiles.contains(entity2) && enemies.contains(entity1) {
        Some((entity2, entity1))
    } else {
        None
    }
}

pub fn handle_combat_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<(Entity, &Projectile)>,
    attack_query: Query<(Entity, &Attack)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health)>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            // Try both orders of entities for projectile hits
            let projectile_hit = check_projectile_hit(&mut commands, e1, e2, &projectile_query, &mut enemy_query)
                || check_projectile_hit(&mut commands, e2, e1, &projectile_query, &mut enemy_query);

            // Try both orders of entities for melee hits
            let melee_hit = check_melee_hit(&mut commands, e1, e2, &attack_query, &mut enemy_query)
                || check_melee_hit(&mut commands, e2, e1, &attack_query, &mut enemy_query);

            if projectile_hit || melee_hit {
                info!("Hit registered!");
            }
        }
    }
}

fn check_projectile_hit(
    commands: &mut Commands,
    proj_entity: &Entity,
    enemy_entity: &Entity,
    projectile_query: &Query<(Entity, &Projectile)>,
    enemy_query: &mut Query<(Entity, &Transform, &mut Health)>,
) -> bool {
    if let Ok((proj_entity, projectile)) = projectile_query.get(*proj_entity) {
        if let Ok((enemy_entity, transform, mut health)) = enemy_query.get_mut(*enemy_entity) {
            health.current -= projectile.damage;
            info!("Projectile hit for {} damage", projectile.damage);
            spawn_floating_text(commands, transform.translation, projectile.damage, Color::WHITE, enemy_entity);
            commands.entity(proj_entity).despawn();
            
            if health.current <= 0.0 {
                commands.entity(enemy_entity).despawn();
            }
            return true;
        }
    }
    false
}

fn check_melee_hit(
    commands: &mut Commands,
    attack_entity: &Entity,
    enemy_entity: &Entity,
    attack_query: &Query<(Entity, &Attack)>,
    enemy_query: &mut Query<(Entity, &Transform, &mut Health)>,
) -> bool {
    if let Ok((attack_entity, attack)) = attack_query.get(*attack_entity) {
        if let Ok((enemy_entity, transform, mut health)) = enemy_query.get_mut(*enemy_entity) {
            health.current -= attack.damage;
            spawn_floating_text(commands, transform.translation, attack.damage, Color::WHITE, enemy_entity);
            
            if health.current <= 0.0 {
                commands.entity(enemy_entity).despawn();
            }
            return true;
        }
    }
    false
}

pub fn cleanup_attacks(
    mut commands: Commands,
    time: Res<Time>,
    mut attack_query: Query<(Entity, &mut Attack)>,
    mut projectile_query: Query<(Entity, &mut Projectile)>,
) {
    // Cleanup melee attacks
    for (entity, mut attack) in attack_query.iter_mut() {
        attack.timer.tick(time.delta());
        if attack.timer.finished() {
            commands.entity(entity).despawn();
        }
    }

    // Cleanup projectiles
    for (entity, mut projectile) in projectile_query.iter_mut() {
        projectile.lifetime.tick(time.delta());
        if projectile.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn handle_attack_hit(
    commands: &mut Commands,
    attack_entity: Entity,
    enemy_entity: Entity,
    attacks: &Query<(Entity, &Attack)>,
    enemies: &mut Query<(Entity, &Transform, &mut Health), With<Enemy>>,
) {
    if let (Ok((_, attack)), Ok((_, transform, mut health))) = (
        attacks.get(attack_entity),
        enemies.get_mut(enemy_entity)
    ) {
        apply_damage(commands, &mut health, attack.damage, transform.translation, enemy_entity);
    }
}

fn handle_projectile_hit(
    commands: &mut Commands,
    proj_entity: Entity,
    enemy_entity: Entity,
    projectiles: &Query<(Entity, &Projectile, Option<&ProjectileType>)>,
    enemies: &mut Query<(Entity, &Transform, &mut Health), With<Enemy>>,
) {
    if let (Ok((_, projectile, proj_type)), Ok((_, transform, mut health))) = (
        projectiles.get(proj_entity),
        enemies.get_mut(enemy_entity)
    ) {
        let damage = if let Some(ProjectileType::Fireball) = proj_type {
            projectile.damage * 1.2
        } else {
            projectile.damage
        };

        apply_damage(commands, &mut health, damage, transform.translation, enemy_entity);
        commands.entity(proj_entity).despawn();
    }
}

fn apply_damage(
    commands: &mut Commands,
    health: &mut Health,
    damage: f32,
    position: Vec3,
    target: Entity,
) {
    health.current -= damage;
    spawn_floating_text(commands, position, damage, Color::WHITE, target);

    if health.current <= 0.0 {
        commands.entity(target).despawn();
    }
}

// pub fn handle_weapon_attack(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut weapon_query: Query<(&mut WeaponItem, &Transform)>,
// ) {
//     for (mut weapon, transform) in weapon_query.iter_mut() {
//         if weapon.can_attack(time.elapsed_secs()) {
//             weapon.set_last_attack(time.elapsed_secs());
            
//             match &weapon.properties {
//                 WeaponProperties::Melee(props) => {
//                     spawn_melee_attack(&mut commands, weapon.damage(), props, transform);
//                 },
//                 WeaponProperties::Ranged(props) => {
//                     spawn_projectile(&mut commands, weapon.damage(), props, transform);
//                 }
//             }
//         }
//     }
// }

// fn spawn_melee_attack(
//     commands: &mut Commands,
//     damage: f32,
//     props: &MeleeProperties,
//     transform: &Transform,
// ) {
//     commands.spawn((
//         SpriteBundle {
//             sprite: Sprite {
//                 color: Color::rgba(1.0, 1.0, 1.0, 0.5),
//                 custom_size: Some(Vec2::new(props.swing_width, props.swing_height)),
//                 ..default()
//             },
//             transform: *transform,
//             ..default()
//         },
//         Attack {
//             timer: Timer::from_seconds(props.swing_time, TimerMode::Once),
//             damage,
//             range: props.swing_width,
//         },
//         RigidBody::KinematicPositionBased,
//         Collider::cuboid(props.swing_width / 2.0, props.swing_height / 2.0),
//     ));
// }

// fn spawn_projectile(
//     commands: &mut Commands,
//     damage: f32,
//     props: &RangedProperties,
//     transform: &Transform,
// ) {
//     commands.spawn((
//         SpriteBundle {
//             sprite: Sprite {
//                 color: Color::rgb(0.8, 0.8, 0.2),
//                 custom_size: Some(Vec2::splat(props.projectile_size)),
//                 ..default()
//             },
//             transform: *transform,
//             ..default()
//         },
//         Projectile {
//             damage,
//             speed: props.projectile_speed,
//             lifetime: Timer::from_seconds(props.max_range / props.projectile_speed, TimerMode::Once),
//         },
//         RigidBody::Dynamic,
//         Collider::ball(props.projectile_size / 2.0),
//         Velocity::linear(Vec2::new(transform.local_x().x, transform.local_x().y) * props.projectile_speed),
//         Sensor,
//     ));
// }

pub fn update_attacks(
    mut commands: Commands,
    time: Res<Time>,
    mut attack_query: Query<(Entity, &mut Attack)>,
) {
    for (entity, mut attack) in attack_query.iter_mut() {
        attack.timer.tick(time.delta());
        if attack.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

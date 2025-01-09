use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
pub struct WeaponItem {
    pub weapon_type: WeaponType,
    pub damage: f32,
    pub attack_speed: f32,
    pub last_attack: f32,
    pub properties: WeaponProperties,
}

#[derive(Debug, Clone, Reflect)]
pub struct MeleeProperties {
    pub swing_width: f32,
    pub swing_height: f32,
    pub swing_time: f32,
    pub swing_arc: f32,
}

#[derive(Debug, Clone, Reflect)]
pub struct RangedProperties {
    pub projectile_speed: f32,
    pub projectile_size: f32,
    pub max_range: f32,
}

#[derive(Debug, Clone, Reflect)]
pub enum WeaponProperties {
    Melee(MeleeProperties),
    Ranged(RangedProperties),
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum WeaponType {
    Melee,
    Ranged
}

#[derive(Component)]
pub struct Attack {
    pub timer: Timer,
    pub damage: f32,
    pub range: f32,
}

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

impl Default for WeaponItem {
    fn default() -> Self {
        Self {
            damage: 10.0,
            attack_speed: 1.0,
            last_attack: 0.0,
            weapon_type: WeaponType::Melee,
            properties: WeaponProperties::Melee(MeleeProperties {
                swing_width: 48.0,
                swing_height: 16.0,
                swing_time: 0.2,
                swing_arc: std::f32::consts::PI * 0.75,
            }),
        }
    }
}

pub trait Weapon: Component {
    fn damage(&self) -> f32;
    fn attack_speed(&self) -> f32;
    fn last_attack(&self) -> f32;
    fn set_last_attack(&mut self, time: f32);
    fn can_attack(&self, current_time: f32) -> bool {
        current_time - self.last_attack() >= 1.0 / self.attack_speed()
    }
}

impl Weapon for WeaponItem {
    fn damage(&self) -> f32 { self.damage }
    fn attack_speed(&self) -> f32 { self.attack_speed }
    fn last_attack(&self) -> f32 { self.last_attack }
    fn set_last_attack(&mut self, time: f32) { self.last_attack = time; }
}

// Replace the old create functions with new ones using WeaponItem
pub fn create_sword() -> WeaponItem {
    WeaponItem {
        weapon_type: WeaponType::Melee,
        damage: 10.0,
        attack_speed: 1.0,
        last_attack: 0.0,
        properties: WeaponProperties::Melee(MeleeProperties {
            swing_width: 48.0,
            swing_height: 16.0,
            swing_time: 0.2,
            swing_arc: std::f32::consts::PI * 0.75,
        }),
    }
}

pub fn create_bow() -> WeaponItem {
    WeaponItem {
        weapon_type: WeaponType::Ranged,
        damage: 8.0,
        attack_speed: 0.8,
        last_attack: 0.0,
        properties: WeaponProperties::Ranged(RangedProperties {
            projectile_speed: 300.0,
            projectile_size: 8.0,
            max_range: 400.0,
        }),
    }
}

// Helper methods for WeaponItem
impl WeaponItem {
    pub fn get_melee_properties(&self) -> Option<&MeleeProperties> {
        match &self.properties {
            WeaponProperties::Melee(props) => Some(props),
            _ => None,
        }
    }

    pub fn get_ranged_properties(&self) -> Option<&RangedProperties> {
        match &self.properties {
            WeaponProperties::Ranged(props) => Some(props),
            _ => None,
        }
    }

    pub fn spawn_attack(&self, commands: &mut Commands, transform: &Transform) {
        match &self.properties {
            WeaponProperties::Melee(props) => {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                            custom_size: Some(Vec2::new(props.swing_width, props.swing_height)),
                            ..default()
                        },
                        transform: *transform,
                        ..default()
                    },
                    Attack {
                        timer: Timer::from_seconds(props.swing_time, TimerMode::Once),
                        damage: self.damage,
                        range: props.swing_width,
                    },
                    RigidBody::KinematicPositionBased,
                    Collider::cuboid(props.swing_width / 2.0, props.swing_height / 2.0),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                ));
            }
            WeaponProperties::Ranged(props) => {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.8, 0.8, 0.2),
                            custom_size: Some(Vec2::splat(props.projectile_size)),
                            ..default()
                        },
                        transform: *transform,
                        ..default()
                    },
                    Projectile {
                        damage: self.damage,
                        speed: props.projectile_speed,
                        lifetime: Timer::from_seconds(props.max_range / props.projectile_speed, TimerMode::Once),
                    },
                    RigidBody::Dynamic,
                    Collider::ball(props.projectile_size / 2.0),
                    Velocity::linear(Vec2::new(transform.local_x().x, transform.local_x().y) * props.projectile_speed),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                ));
            }
        }
    }
}
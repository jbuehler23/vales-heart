use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub damage: f32,
    pub attack_range: f32,
    pub attack_speed: f32,
    pub last_attack: f32,
}

#[derive(Debug, Clone, Copy)]
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
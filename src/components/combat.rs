use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// src/components/combat.rs
#[derive(Component, Debug)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
}

#[derive(Component, Debug)]
pub struct Enemy {
    pub damage: f32,
    pub detection_range: f32,
}

#[derive(Component, Debug)]
pub struct CombatDebug;

#[derive(Component)]
pub struct HitEffect {
    pub damage: f32,
    pub lifetime: Timer,
    pub offset: Vec2,
}

#[derive(Component)]
pub struct DamageNumber {
    pub value: f32,
    pub lifetime: Timer,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct FloatingText {
    pub lifetime: Timer,
    pub velocity: Vec2,
    pub fade: f32,
    pub initial_y_offset: f32,
    pub gravity: f32,  // Add gravity for arc motion
    pub parent: Entity,  // Add parent entity reference
}
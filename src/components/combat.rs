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
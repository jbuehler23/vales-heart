use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub facing: Direction,
}

#[derive(Component)]
pub struct MovementInput {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
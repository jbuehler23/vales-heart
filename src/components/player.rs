use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub facing: Direction,
    pub character_stats: CharacterStats,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 200.0,  // Increased base speed
            facing: Direction::Down,
            character_stats: CharacterStats::default(),
        }
    }
}

#[derive(Component, Debug)]
pub struct CharacterStats {
    pub health: f32,
    pub max_health: f32,
    pub mana: f32,
    pub max_mana: f32,
    pub level: u32,
    pub experience: u32,
}

impl Default for CharacterStats {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            mana: 100.0,
            max_mana: 100.0,
            level: 1,
            experience: 0,
        }
    }
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
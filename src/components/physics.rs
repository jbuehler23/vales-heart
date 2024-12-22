use bevy::prelude::*;

#[derive(Component)]
pub struct CollisionDebug {
    pub show_hitbox: bool,
    pub show_direction: bool,
}

#[derive(Component)]
pub struct MovementTrail {
    pub points: Vec<Vec2>,
    pub max_points: usize,
}

impl Default for MovementTrail {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            max_points: 50,
        }
    }
}
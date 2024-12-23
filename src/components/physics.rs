use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// Debug components to control visualization features
#[derive(Component)]
pub struct DebugShape {
    pub color: Color,
    pub shape_type: DebugShapeType,
}

#[derive(Component)]
pub struct MovementTrail {
    pub points: Vec<Vec2>,
    pub max_points: usize,
    pub trail_color: Color,
}

#[derive(Debug, Clone, Copy)]
pub enum DebugShapeType {
    Hitbox,
    DirectionArrow,
    TrailPoint,
}

impl Default for MovementTrail {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            max_points: 50,
            trail_color: Color::rgba(0.0, 0.0, 1.0, 0.5),
        }
    }
}
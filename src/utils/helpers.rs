use bevy::prelude::*;

pub fn vector_to_direction(vector: Vec2) -> Option<crate::components::player::Direction> {
    if vector.length() == 0.0 {
        return None;
    }
    
    let angle = vector.y.atan2(vector.x);
    match angle {
        a if a.abs() <= std::f32::consts::FRAC_PI_4 => Some(crate::components::player::Direction::Right),
        a if a.abs() >= 3.0 * std::f32::consts::FRAC_PI_4 => Some(crate::components::player::Direction::Left),
        a if a > 0.0 => Some(crate::components::player::Direction::Up),
        _ => Some(crate::components::player::Direction::Down),
    }
}
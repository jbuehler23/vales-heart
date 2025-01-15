use bevy::prelude::*;

#[derive(Component)]
pub struct SpriteAnimation {
    pub frames: Vec<Handle<Image>>,
    pub current_frame: usize,
    pub timer: Timer,
    pub is_moving: bool,
    pub facing_left: bool,
}

impl Default for SpriteAnimation {
    fn default() -> Self {
        Self {
            frames: Vec::new(),
            current_frame: 0,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            is_moving: false,
            facing_left: false,
        }
    }
}
use bevy::prelude::*;
use crate::components::player::*;
// use crate::components::stats::*;
use crate::systems::{input::player_input, movement::player_movement};
use crate::utils::constants::PLAYER_SPEED;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (player_input, player_movement));
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            Sprite {
                color: Color::srgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1.0),
        ))
        .insert(Player {
            speed: PLAYER_SPEED,
            facing: Direction::Down,
        })
        .insert(MovementInput { x: 0.0, y: 0.0 });
        // .insert(Stats::default());
}

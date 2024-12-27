use crate::{components::{combat::Enemy, player::*}, resources::GameState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, CoefficientCombineRule, Collider, Friction, GravityScale, LockedAxes, RigidBody, Velocity};
// use crate::components::stats::*;
use crate::systems::{input::player_input, movement::player_movement};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_input, player_movement.after(player_input))
            .run_if(in_state(GameState::Playing)));
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player {
            speed: 150.0,
            facing: Direction::Down,
            character_stats: CharacterStats::default(),
        },
        MovementInput { x: 0.0, y: 0.0 },
        // Add physics components
        RigidBody::Dynamic,
        Collider::cuboid(16.0, 16.0), // Half-extents for the 32x32 sprite
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED, // Prevent rotation
        GravityScale(0.0), // Prevent gravity
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        ActiveEvents::COLLISION_EVENTS,
    ));
}
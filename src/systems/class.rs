use bevy::prelude::*;
use crate::{
    components::{class::*, player::*, weapon::*},
    resources::GameState,
};
use bevy_rapier2d::prelude::*;

pub fn spawn_selected_player(
    commands: Commands,
    selected_class: Res<SelectedClass>,
) {
    if let Some(class_type) = selected_class.class_type {
        spawn_player_with_class(commands, class_type);
    }
}

pub fn spawn_player_with_class(
    mut commands: Commands,
    class_type: ClassType,
) {
    let (player_class, weapon) = PlayerClass::new(class_type);
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        player_class.clone(),
        weapon,
        Player {
            speed: player_class.base_stats.speed,
            facing: Direction::Down,
            character_stats: CharacterStats::default(),
        },
        MovementInput { x: 0.0, y: 0.0 },
        RigidBody::Dynamic,
        Collider::cuboid(16.0, 16.0),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(0.0),
        Velocity::zero(),
        ActiveEvents::COLLISION_EVENTS,
    ));
}

pub fn handle_class_selection(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        spawn_player_with_class(commands, ClassType::Warrior);
        next_state.set(GameState::Playing);
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        spawn_player_with_class(commands, ClassType::Archer);
        next_state.set(GameState::Playing);
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        spawn_player_with_class(commands, ClassType::Mage);
        next_state.set(GameState::Playing);
    }
}
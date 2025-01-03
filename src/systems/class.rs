use bevy::prelude::*;
use crate::{
    components::{class::*, inventory::Equipment, player::*, weapon::*},
    resources::GameState,
};

use bevy_rapier2d::prelude::*;

// Add class-based player system
pub fn spawn_selected_player(
    mut commands: Commands,
    selected_class: Res<SelectedClass>,
) {
    if let Some(class_type) = selected_class.class_type {
        let (player_class, weapon) = PlayerClass::new(class_type);
        
        // Spawn player without RigidBody
        let player = commands.spawn((
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
            Player {
                speed: 200.0,
                facing: Direction::Down,
                character_stats: CharacterStats::default(),
                equipment: Equipment::default(),
            },
            MovementInput { x: 0.0, y: 0.0 },
            Collider::cuboid(16.0, 16.0),
            ActiveEvents::COLLISION_EVENTS,
        )).id();

        // Spawn weapon as child
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: match class_type {
                        ClassType::Warrior => Color::from(Srgba::GREEN),
                        ClassType::Archer => Color::from(Srgba::BLUE),
                        ClassType::Mage => Color::from(Srgba::BLACK),
                    },
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                transform: Transform::from_xyz(16.0, 0.0, 0.0),
                ..default()
            },
            weapon,
            Collider::cuboid(8.0, 8.0),
            ActiveEvents::COLLISION_EVENTS,
        )).set_parent(player);
    }
}

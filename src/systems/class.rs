use bevy::prelude::*;
use crate::components::{assets::GameAssets, class::*, inventory::Equipment, player::*};

use bevy_rapier2d::prelude::*;

// Add class-based player system
pub fn spawn_selected_player(
    mut commands: Commands,
    selected_class: Res<SelectedClass>,
    game_assets: Res<GameAssets>,
) {
    
    if let Some(class_type) = selected_class.class_type {
        let frames = match class_type {
            ClassType::Warrior => game_assets.warrior_walk.clone(),
            ClassType::Archer => game_assets.archer_walk.clone(),
            ClassType::Mage => game_assets.mage_walk.clone(),
        };
        let (player_class, weapon) = PlayerClass::new(class_type);
        
        // Spawn player without RigidBody
        let player = commands.spawn((
            Sprite::from_image(frames[0].clone()),
            player_class.clone(),
            Player::default(),
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

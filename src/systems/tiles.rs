// use bevy::prelude::*;

// // Update our tile spawn system to include physics
// pub fn spawn_tile(
//     commands: &mut Commands,
//     x: i32,
//     y: i32,
//     tile_type: TileType,
// ) {
//     let is_solid = matches!(tile_type, TileType::Wall);
    
//     commands.spawn((
//         SpriteBundle {
//             sprite: Sprite {
//                 color: tile_type.get_color(),
//                 custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
//                 ..default()
//             },
//             transform: Transform::from_xyz(
//                 x as f32 * TILE_SIZE,
//                 y as f32 * TILE_SIZE,
//                 0.0
//             ),
//             ..default()
//         },
//         Tile {
//             tile_type,
//             position: GridPosition { x, y },
//         },
//         // Add physics components for solid tiles
//         if is_solid {
//             RigidBody::Fixed
//         } else {
//             RigidBody::Sensor
//         },
//         Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
//     ));
// }
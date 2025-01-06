use bevy::prelude::{Commands, Transform};

use crate::systems::world::spawn_random_item;

pub fn drop_item(drop_chance: f32, enemy_transform: &Transform, mut commands: &mut Commands) {
    // 50% chance to drop an item
    if rand::random::<f32>() < drop_chance {
        spawn_random_item(
            &mut commands,
            enemy_transform.translation,
            None // Random rarity
        );
    }
}
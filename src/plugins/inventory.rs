use bevy::prelude::*;
use crate::{components::inventory::{Inventory, Item, ItemStack}, systems::inventory::*};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Inventory>()
            .register_type::<ItemStack>()
            .register_type::<Item>()
            .add_systems(Update, update_inventory_ui);
    }
}
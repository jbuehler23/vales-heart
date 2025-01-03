use bevy::prelude::*;
use crate::{components::{inventory::{Inventory, Item, ItemStack}, ui::InventoryState}, systems::{input::toggle_inventory, inventory::*, ui::{spawn_inventory_ui, update_inventory_slots}}};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Inventory>()
            .register_type::<ItemStack>()
            .register_type::<Item>()
            .insert_resource(InventoryState::default())
            .add_systems(Startup, spawn_inventory_ui)
            .add_systems(Update, (
                update_inventory_ui,
            toggle_inventory,
        update_inventory_slots));
    }
}
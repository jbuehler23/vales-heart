use bevy::prelude::*;
use crate::{components::{
    armor::ArmorItem, consumable::ConsumableItem, inventory::*, item::{Item, ItemRarity, ItemType}, player::{CharacterStats, Effect}, ui::InventoryState, weapon::WeaponItem
}, systems::{input::toggle_inventory, inventory::{handle_item_pickup, handle_item_use, update_equipment_stats, update_inventory_slots, update_inventory_ui}, ui::spawn_inventory_ui, world::update_item_hover_text}};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Item>()
            .register_type::<ItemType>()
            .register_type::<ItemRarity>()
            .register_type::<WeaponItem>()
            .register_type::<ArmorItem>() 
            .register_type::<ConsumableItem>()
            .register_type::<Equipment>()
            .register_type::<CharacterStats>()
            .register_type::<Effect>()
            .insert_resource(InventoryState::default())
            .add_systems(Startup, spawn_inventory_ui)
            .add_systems(Update, (
                update_inventory_ui,
                toggle_inventory,
                update_inventory_slots,
                handle_item_pickup,
                handle_item_use,
                update_effects,
                update_equipment_stats,
                update_item_hover_text,
            ));
    }
}

fn update_effects(mut query: Query<&mut CharacterStats>, time: Res<Time>) {
    for mut stats in query.iter_mut() {
        stats.active_effects.retain(|_, effect| {
            effect.duration -= time.delta_secs();
            effect.duration > 0.0
        });
    }
}
use bevy::prelude::*;
use crate::components::{consumable::EffectType, inventory::*, item::ItemRarity};

pub fn add_item_to_inventory(
    inventory: &mut Inventory,
    item_stack: ItemStack,
) -> Result<(), String> {
    // Find first empty slot
    for slot in 0..inventory.capacity {
        if !inventory.slots.contains_key(&slot) {
            inventory.slots.insert(slot, Some(item_stack));
            return Ok(());
        }
    }
    Err("Inventory is full".to_string())
}

pub fn remove_item_from_inventory(
    inventory: &mut Inventory,
    slot: usize,
) -> Result<ItemStack, String> {
    if let Some(Some(item)) = inventory.slots.remove(&slot) {
        Ok(item)
    } else {
        Err("No item in slot".to_string())
    }
}

pub fn update_inventory_ui(
    query: Query<(&Inventory, &Children), Changed<Inventory>>,
    mut ui_query: Query<&mut Text>,
) {
    for (inventory, children) in query.iter() {
        // Update UI representation of inventory
        // Implementation depends on your UI setup
    }
}

use bevy::prelude::*;
use crate::components::{
    inventory::*,
    item::{Item, ItemType},
    ui::InventoryUI,
    player::Player,
};

// Handle picking up items in the world
pub fn handle_item_pickup(
    mut commands: Commands,
    mut player_query: Query<&mut Inventory, With<Player>>,
    item_query: Query<(Entity, &Item), Added<Item>>,
) {
    if let Ok(mut inventory) = player_query.get_single_mut() {
        for (item_entity, item) in item_query.iter() {
            let item_stack = ItemStack {
                item: item.clone(),
                quantity: 1,
            };

            match add_item_to_inventory(&mut inventory, item_stack) {
                Ok(_) => {
                    // Remove item from world
                    commands.entity(item_entity).despawn();
                }
                Err(_) => {
                    // Inventory full - leave item in world
                }
            }
        }
    }
}

// Handle using items from inventory
pub fn handle_item_use(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Inventory, &mut Player)>,
) {
    if let Ok((mut inventory, mut player)) = player_query.get_single_mut() {
        // Example: Use item in first slot with Q key
        if keyboard.just_pressed(KeyCode::KeyQ) {
            if let Ok(item_stack) = remove_item_from_inventory(&mut inventory, 0) {
                match item_stack.item.item_type {
                    ItemType::Consumable(consumable) => {
                        // Apply consumable effects
                        match consumable.effect_type {
                            EffectType::Health => {
                                // Heal player
                            }
                            EffectType::Mana => {
                                // Restore mana
                            }
                            EffectType::Stamina => {
                                // Restore stamina  
                            }
                            EffectType::StatBuff(_) => {
                                // Apply buff
                            }
                        }
                    }
                    ItemType::Weapon(weapon) => {
                        // Equip weapon
                    }
                    ItemType::Armor(armor) => {
                        // Equip armor
                    }
                }
            }
        }
    }
}

// Update inventory slot UI
pub fn update_inventory_slots(
    inventory_query: Query<(&Inventory, &Children)>,
    mut slot_query: Query<(Entity, &mut BackgroundColor, &mut Text), With<InventoryUI>>,
) {
    for (inventory, _) in inventory_query.iter() {
        for (slot_index, item_stack) in inventory.slots.iter() {
            for (entity, mut bg_color, mut text) in slot_query.iter_mut() {
                if let Some(item_stack) = item_stack {
                    // Update slot appearance based on item
                    bg_color.0 = match item_stack.item.rarity {
                        ItemRarity::Common => Color::srgb(0.5, 0.5, 0.5),
                        ItemRarity::Uncommon => Color::srgb(0.2, 0.8, 0.2),
                        ItemRarity::Rare => Color::srgb(0.2, 0.2, 0.8),
                        ItemRarity::Epic => Color::srgb(0.8, 0.2, 0.8),
                        ItemRarity::Legendary => Color::srgb(0.8, 0.8, 0.2),
                    };
                    text.0 = item_stack.item.name.clone();
                } else {
                    // Empty slot
                    bg_color.0 = Color::srgba(0.2, 0.2, 0.2, 0.5);
                    text.0 = String::new();
                }
            }
        }
    }
}
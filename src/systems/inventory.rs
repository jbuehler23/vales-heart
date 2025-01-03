use crate::components::{armor::ArmorSlot, consumable::EffectType, inventory::*, item::ItemRarity, player::{CharacterStats, Effect}};
use bevy::prelude::*;

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

use crate::components::{
    inventory::*,
    item::{Item, ItemType},
    player::Player,
    ui::InventoryUI,
};
use bevy::prelude::*;

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
                    // Successfully added item to inventory, remove from world
                    commands.entity(item_entity).despawn();
                }
                Err(e) => {
                    info!("Could not pick up item: {}", e);
                    // Leave item in world
                }
            }
        }
    }
}

// Handle using/equipping items
pub fn handle_item_use(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Inventory, &mut Player)>,
) {
    if let Ok((mut inventory, mut player)) = player_query.get_single_mut() {
        // Use item in first slot with 1 key
        if keyboard.just_pressed(KeyCode::KeyQ) {
            if let Ok(item_stack) = remove_item_from_inventory(&mut inventory, 0) {
                match item_stack.item.item_type {
                    ItemType::Consumable(consumable) => {
                        apply_consumable_effect(&mut player, consumable);
                    }
                    ItemType::Weapon(weapon) => {
                        equip_weapon(&mut player, weapon);
                    }
                    ItemType::Armor(armor) => {
                        equip_armor(&mut player, armor);
                    }
                }
            }
        }
    }
}

fn equip_armor(player: &mut Player, armor: crate::components::armor::ArmorItem) {
    match armor.slot {
        ArmorSlot::Head => player.equipment.head = Some(armor),
        ArmorSlot::Chest => player.equipment.chest = Some(armor),
        ArmorSlot::Legs => player.equipment.legs = Some(armor),
        ArmorSlot::Feet => player.equipment.feet = Some(armor),
    }
}

fn equip_weapon(player: &mut Player, weapon: crate::components::weapon::WeaponItem) {
    player.equipment.weapon = Some(weapon);
}

fn apply_consumable_effect(player: &mut Player, consumable: crate::components::consumable::ConsumableItem) {
    let effect_type = consumable.effect_type;
    match effect_type {
        EffectType::Health => {
            let current_health = player.character_stats.health;
            player.character_stats.health = (current_health + consumable.potency).min(player.character_stats.max_health);
        }
        EffectType::Mana => {
            let current_mana = player.character_stats.mana;
            player.character_stats.mana = (current_mana + consumable.potency).min(player.character_stats.max_mana);
        }
        EffectType::StatBuff(buff_type) => {
            player.character_stats.active_effects.insert(
                buff_type.to_string(),
                Effect {
                    duration: consumable.duration.unwrap_or(0.0),
                    potency: consumable.potency,
                    effect_type: effect_type,
                }
            );
        }
        EffectType::Stamina => {
            let current_stamina = player.character_stats.stamina;
            player.character_stats.stamina = (current_stamina + consumable.potency).min(player.character_stats.max_stamina);
        }
    }
}

// Update UI to show inventory contents
pub fn update_inventory_slots(
    inventory_query: Query<(&Inventory, &Children)>,
    mut slot_query: Query<(Entity, &mut BackgroundColor, &mut Text), With<InventoryUI>>,
) {
    for (inventory, _children) in inventory_query.iter() {
        // Create a map of slot indices to slot entities
        let slot_entities: Vec<_> = slot_query.iter().map(|(e, _, _)| e).collect();
        for (slot_index, item_stack) in inventory.slots.iter() {
            if *slot_index < slot_entities.len() {
                if let Ok((_, mut bg_color, mut text)) =
                    slot_query.get_mut(slot_entities[*slot_index])
                {
                    if let Some(item_stack) = item_stack {
                        // Update slot appearance based on item
                        *bg_color = get_rarity_color(&item_stack.item.rarity).into();
                        text.0 = format!("{} ({})", item_stack.item.name, item_stack.quantity);
                    } else {
                        // Empty slot
                        *bg_color = Color::srgba(0.2, 0.2, 0.2, 0.5).into();
                        text.0 = String::new();
                    }
                }
            }
        }
    }

    // Helper functions
    fn get_rarity_color(rarity: &ItemRarity) -> Color {
        match rarity {
            ItemRarity::Common => Color::srgb(0.5, 0.5, 0.5),
            ItemRarity::Uncommon => Color::srgb(0.2, 0.8, 0.2),
            ItemRarity::Rare => Color::srgb(0.2, 0.2, 0.8),
            ItemRarity::Epic => Color::srgb(0.8, 0.2, 0.8),
            ItemRarity::Legendary => Color::srgb(0.8, 0.8, 0.2),
        }
    }
}

/// Update player stats based on equipped items
pub fn update_equipment_stats(mut query: Query<(&Equipment, &mut CharacterStats)>) {
    for (equipment, mut stats) in query.iter_mut() {
        // Reset to base stats first
        stats.reset_to_base();
        
        // Apply weapon bonuses if equipped
        if let Some(weapon) = &equipment.weapon {
            stats.apply_weapon_bonus(weapon);
        }

        // Apply armor bonuses from each slot
        if let Some(head) = &equipment.head {
            stats.apply_armor_bonus(head);
        }
        if let Some(chest) = &equipment.chest {
            stats.apply_armor_bonus(chest);
        }
        if let Some(legs) = &equipment.legs {
            stats.apply_armor_bonus(legs);
        }
        if let Some(feet) = &equipment.feet {
            stats.apply_armor_bonus(feet);
        }
    }
}

use bevy::prelude::*;
use crate::components::inventory::*;

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
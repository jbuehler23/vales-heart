use bevy::prelude::*;
use std::collections::HashMap;

use super::{armor::ArmorItem, consumable::ConsumableItem, weapon::WeaponItem};

#[derive(Component, Clone, Reflect)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub rarity: ItemRarity,
    pub value: u32,
}

#[derive(Clone, Reflect)]
pub enum ItemType {
    Weapon(WeaponItem),
    Armor(ArmorItem),
    Consumable(ConsumableItem),
}

#[derive(Clone, Reflect)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}
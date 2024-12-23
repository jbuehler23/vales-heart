use bevy::prelude::Component;

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: usize,
}

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub stats: ItemStats,
}

#[derive(Clone)]
pub struct ItemStats {
    pub health: f32,
    pub attack: f32,
    pub defense: f32,
}

#[derive(Clone)]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
}
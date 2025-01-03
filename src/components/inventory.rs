use bevy::prelude::*;
use std::collections::HashMap;

use super::{item::Item, weapon::Weapon};

#[derive(Component, Reflect)]
pub struct Inventory {
    pub slots: HashMap<usize, Option<ItemStack>>,
    pub capacity: usize,
}

#[derive(Component, Reflect)]
pub struct ItemStack {
    pub item: Item,
    pub quantity: u32,
}

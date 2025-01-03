use bevy::prelude::*;
use std::collections::HashMap;

use super::{armor::ArmorItem, item::Item, weapon::WeaponItem};

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


#[derive(Component, Default, Reflect)]
pub struct Equipment {
    pub weapon: Option<WeaponItem>,
    pub head: Option<ArmorItem>,
    pub chest: Option<ArmorItem>,
    pub legs: Option<ArmorItem>,
    pub feet: Option<ArmorItem>,
}

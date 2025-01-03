use std::collections::HashMap;

use bevy::reflect::Reflect;

#[derive(Clone, Reflect)]
pub enum ArmorSlot {
    Head,
    Chest,
    Legs,
    Feet,
}

#[derive(Clone, Reflect)]
pub struct ArmorItem {
    pub defense: f32,
    pub armor_type: ArmorType,
    pub slot: ArmorSlot,
    pub requirements: HashMap<String, f32>,
}


#[derive(Clone, Reflect)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
}


#[derive(Clone, Reflect)]
pub struct ArmorStats {
    pub defense: f32,
    pub armor_type: ArmorType,
}
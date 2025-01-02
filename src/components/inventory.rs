use bevy::prelude::*;
use std::collections::HashMap;

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

#[derive(Component, Clone, Reflect)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub item_type: ItemType,
    pub rarity: ItemRarity,
    pub value: u32,
    pub weight: f32,
}

#[derive(Clone, Reflect)]
pub enum ItemType {
    Weapon(WeaponStats),
    Armor(ArmorStats),
    Consumable(ConsumableStats),
    Material,
    Quest,
}

#[derive(Clone, Reflect)]
pub struct WeaponStats {
    pub damage: f32,
    pub attack_speed: f32,
    pub weapon_type: WeaponType,
}

#[derive(Clone, Reflect)]
pub struct ArmorStats {
    pub defense: f32,
    pub armor_type: ArmorType,
}

#[derive(Clone, Reflect)]
pub struct ConsumableStats {
    pub effect_type: EffectType,
    pub effect_value: f32,
    pub duration: Option<f32>,
}

#[derive(Clone, Reflect)]
pub enum WeaponType {
    Sword,
    Bow,
    Staff,
    // Add more weapon types
}

#[derive(Clone, Reflect)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
}

#[derive(Clone, Reflect)]
pub enum EffectType {
    Heal,
    ManaRestore,
    StaminaRestore,
    TemporaryBuff(BuffType),
}

#[derive(Clone, Reflect)]
pub enum BuffType {
    Damage,
    Defense,
    Speed,
}

#[derive(Clone, Reflect)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}
use std::collections::HashMap;

use bevy::prelude::*;

use super::{armor::ArmorItem, consumable::EffectType, inventory::Equipment, weapon::WeaponItem};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub facing: Direction,
    pub character_stats: CharacterStats,
    pub equipment: Equipment,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 200.0,  // Increased base speed
            facing: Direction::Down,
            character_stats: CharacterStats::default(),
            equipment: Equipment::default(),
        }
    }
}

#[derive(Component, Reflect)]
pub struct CharacterStats {
    pub health: f32,
    pub max_health: f32,
    pub mana: f32,
    pub max_mana: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub level: u32,
    pub experience: u32,
    pub active_effects: HashMap<String, Effect>,
}

#[derive(Clone, Reflect)]
pub struct Effect {
    pub duration: f32,
    pub potency: f32,
    pub effect_type: EffectType,
}


impl Default for CharacterStats {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            mana: 100.0,
            max_mana: 100.0,
            stamina: 100.0,
            max_stamina: 100.0,
            level: 1,
            experience: 0,
            active_effects: HashMap::new(),
        }
    }
}

impl CharacterStats {
    pub fn reset_to_base(&mut self) {
        // Reset derived stats but keep base values
        self.health = self.max_health;
        self.mana = self.max_mana;
        self.stamina = self.max_stamina;
    }

    pub fn apply_weapon_bonus(&mut self, weapon: &WeaponItem) {
        // Apply weapon stat bonuses
        // Example: Weapon might give bonus to attack speed, damage, etc
    }

    pub fn apply_armor_bonus(&mut self, armor: &ArmorItem) {
        // Apply armor stat bonuses
        self.max_health += armor.defense * 2.0; // Example: Each point of defense gives 2 HP
        // Could also affect other stats like stamina regen, mana, etc
    }
}

#[derive(Component)]
pub struct MovementInput {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use bevy::prelude::*;

use super::weapon::WeaponType;

#[derive(Component, Debug, Clone)]
pub struct PlayerClass {
    pub class_type: ClassType,
    pub primary_weapon: WeaponStats,
    pub class_abilities: Vec<Ability>,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum ClassType {
    Warrior,
    Archer,
    Mage,
}

#[derive(Debug, Clone)]
pub struct WeaponStats {
    pub name: String,
    pub weapon_type: WeaponType,
    pub base_damage: f32,
    pub attack_speed: f32,
    pub range: f32,
}

#[derive(Debug, Clone)]
pub struct Ability {
    pub name: String,
    pub cooldown: f32,
    pub damage: f32,
    pub range: f32,
}

impl PlayerClass {
    pub fn warrior() -> Self {
        Self {
            class_type: ClassType::Warrior,
            primary_weapon: WeaponStats {
                name: "Sword".to_string(),
                weapon_type: WeaponType::Melee,
                base_damage: 15.0,
                attack_speed: 1.0,
                range: 32.0,
            },
            class_abilities: vec![
                Ability {
                    name: "Whirlwind".to_string(),
                    cooldown: 5.0,
                    damage: 25.0,
                    range: 48.0,
                },
            ],
        }
    }

    pub fn archer() -> Self {
        Self {
            class_type: ClassType::Archer,
            primary_weapon: WeaponStats {
                name: "Bow".to_string(),
                weapon_type: WeaponType::Ranged,
                base_damage: 10.0,
                attack_speed: 0.8,
                range: 150.0,
            },
            class_abilities: vec![
                Ability {
                    name: "Multi-Shot".to_string(),
                    cooldown: 8.0,
                    damage: 20.0,
                    range: 120.0,
                },
            ],
        }
    }

    pub fn mage() -> Self {
        Self {
            class_type: ClassType::Mage,
            primary_weapon: WeaponStats {
                name: "Staff".to_string(),
                weapon_type: WeaponType::Ranged,
                base_damage: 8.0,
                attack_speed: 1.2,
                range: 100.0,
            },
            class_abilities: vec![
                Ability {
                    name: "Fireball".to_string(),
                    cooldown: 3.0,
                    damage: 30.0,
                    range: 200.0,
                },
            ],
        }
    }
}
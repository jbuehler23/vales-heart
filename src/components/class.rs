use bevy::prelude::*;
use super::weapon::{Weapon, WeaponType};

#[derive(Component, Debug, Clone)]
pub struct PlayerClass {
    pub class_type: ClassType,
    pub base_stats: ClassStats,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum ClassType {
    Warrior,
    Archer,
    Mage,
}

#[derive(Debug, Clone)]
pub struct ClassStats {
    pub health: f32,
    pub speed: f32,
    pub damage_multiplier: f32,
}

impl PlayerClass {
    pub fn new(class_type: ClassType) -> (Self, Weapon) {
        match class_type {
            ClassType::Warrior => (
                Self {
                    class_type,
                    base_stats: ClassStats {
                        health: 100.0,
                        speed: 100.0,
                        damage_multiplier: 1.2,
                    },
                },
                Weapon {
                    weapon_type: WeaponType::Melee,
                    damage: 15.0,
                    attack_range: 32.0,
                    attack_speed: 1.0,
                    last_attack: 0.0,
                }
            ),
            ClassType::Archer => (
                Self {
                    class_type,
                    base_stats: ClassStats {
                        health: 80.0,
                        speed: 120.0,
                        damage_multiplier: 1.0,
                    },
                },
                Weapon {
                    weapon_type: WeaponType::Ranged,
                    damage: 10.0,
                    attack_range: 150.0,
                    attack_speed: 0.8,
                    last_attack: 0.0,
                }
            ),
            ClassType::Mage => (
                Self {
                    class_type,
                    base_stats: ClassStats {
                        health: 70.0,
                        speed: 90.0,
                        damage_multiplier: 1.5,
                    },
                },
                Weapon {
                    weapon_type: WeaponType::Ranged,
                    damage: 8.0,
                    attack_range: 100.0,
                    attack_speed: 1.2,
                    last_attack: 0.0,
                }
            ),
        }
    }
}
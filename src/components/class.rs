use bevy::prelude::*;
use super::weapon::{WeaponItem, WeaponType, WeaponProperties, MeleeProperties, RangedProperties};

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
    pub fn new(class_type: ClassType) -> (Self, WeaponItem) {
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
                WeaponItem {
                    weapon_type: WeaponType::Melee,
                    damage: 15.0,
                    attack_speed: 1.0,
                    last_attack: 0.0,
                    properties: WeaponProperties::Melee(MeleeProperties {
                        swing_width: 48.0,
                        swing_height: 24.0,
                        swing_time: 0.3,
                        swing_arc: std::f32::consts::PI * 0.75,
                    }),
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
                WeaponItem {
                    weapon_type: WeaponType::Ranged,
                    damage: 10.0,
                    attack_speed: 0.8,
                    last_attack: 0.0,
                    properties: WeaponProperties::Ranged(RangedProperties {
                        projectile_speed: 400.0,
                        projectile_size: 8.0,
                        max_range: 500.0,
                    }),
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
                WeaponItem {
                    weapon_type: WeaponType::Ranged,
                    damage: 8.0,
                    attack_speed: 1.2,
                    last_attack: 0.0,
                    properties: WeaponProperties::Ranged(RangedProperties {
                        projectile_speed: 300.0,
                        projectile_size: 12.0,
                        max_range: 400.0,
                    }),
                }
            ),
        }
    }

    pub fn modify_weapon(&self, weapon: &mut WeaponItem) {
        match (self.class_type, &weapon.properties) {
            (ClassType::Warrior, WeaponProperties::Melee(_)) => {
                weapon.damage *= self.base_stats.damage_multiplier;
                weapon.attack_speed *= 1.2;
            },
            (ClassType::Archer, WeaponProperties::Ranged(props)) => {
                weapon.damage *= self.base_stats.damage_multiplier;
                if let WeaponProperties::Ranged(ref mut ranged) = weapon.properties {
                    ranged.projectile_speed *= 1.3;
                    ranged.max_range *= 1.2;
                }
            },
            (ClassType::Mage, WeaponProperties::Ranged(_)) => {
                weapon.damage *= self.base_stats.damage_multiplier;
                if let WeaponProperties::Ranged(ref mut ranged) = weapon.properties {
                    ranged.projectile_size *= 1.5;
                }
            },
            _ => {} // No special modifiers for mismatched class/weapon combinations
        }
    }
}

#[derive(Resource)]
pub struct SelectedClass {
    pub class_type: Option<ClassType>,
}

impl Default for SelectedClass {
    fn default() -> Self {
        Self { class_type: None }
    }
}
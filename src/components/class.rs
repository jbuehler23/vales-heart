use bevy::prelude::*;
use super::weapon::{create_bow, create_sword, create_wand, WeaponItem, WeaponProperties, WeaponType};

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
                create_sword(),
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
                create_bow(),
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
                create_wand(),
            ),
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
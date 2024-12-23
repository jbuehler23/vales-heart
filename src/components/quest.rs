use bevy::prelude::Component;

use super::inventory::Item;

// src/components/quest.rs
#[derive(Component)]
pub struct QuestLog {
    pub active_quests: Vec<Quest>,
    pub completed_quests: Vec<Quest>,
}

#[derive(Component)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub objectives: Vec<Objective>,
    pub rewards: Vec<Reward>,
}

#[derive(Component)]
pub struct Objective {
    pub description: String,
    pub is_complete: bool,
}

#[derive(Component)]
pub struct Reward {
    pub reward_type: Vec<RewardType>,
    pub xp_amount: f32,
    pub currency_amount: f32,
    pub item: Item,
}

#[derive(Debug, Clone)]
pub enum RewardType {
    Item,
    Currency,
    Experience,
}
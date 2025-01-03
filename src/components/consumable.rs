use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Consumable {
    pub effect_type: EffectType,
    pub effect_value: f32,
    pub duration: Option<f32>,
}

#[derive(Clone, Reflect)]
pub enum EffectType {
    Health,
    Mana,
    Stamina,
    StatBuff(StatType),
}

#[derive(Clone, Reflect)]
pub enum StatType {
    Damage,
    Defense,
    Speed,
}

#[derive(Clone, Reflect)]
pub struct ConsumableItem {
    pub effect_type: EffectType,
    pub potency: f32,
    pub duration: Option<f32>,
}
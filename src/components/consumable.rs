use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Consumable {
    pub effect_type: EffectType,
    pub effect_value: f32,
    pub duration: Option<f32>,
}

#[derive(Copy, Clone, Reflect)]
pub enum EffectType {
    Health,
    Mana,
    Stamina,
    StatBuff(StatType),
}

#[derive(Clone, Reflect, Copy)]
pub enum StatType {
    Damage,
    Defense,
    Speed,
}
impl StatType {
    pub(crate) fn to_string(&self) -> String {
        match self {
            StatType::Damage => "Damage".to_string(),
            StatType::Defense => "Defense".to_string(),
            StatType::Speed => "Speed".to_string(),
        }
    }
}

#[derive(Clone, Reflect)]
pub struct ConsumableItem {
    pub effect_type: EffectType,
    pub potency: f32,
    pub duration: Option<f32>,
}
use bevy::prelude::Component;

#[derive(Component)]
pub struct NPC {
    pub name: String,
    pub dialogue_id: String,
    pub quest_giver: bool,
}
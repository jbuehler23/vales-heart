use bevy::prelude::{Entity, Resource};

#[derive(Default, Resource)]
pub struct MenuData {
    pub root_entity: Option<Entity>,
}
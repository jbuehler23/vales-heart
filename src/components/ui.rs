use bevy::prelude::{Component, Entity, Resource};

#[derive(Default, Resource)]
pub struct MenuData {
    pub root_entity: Option<Entity>,
}

#[derive(Component)]
pub struct MenuButton {
    pub button_type: ButtonType,
}

#[derive(Clone, Copy)]
pub enum ButtonType {
    Resume,
    Restart,
}


impl From<String> for ButtonType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Resume" => ButtonType::Resume,
            "Restart" => ButtonType::Restart,
            _ => panic!("Invalid button type"),
        }
    }
}

impl ToString for ButtonType {
    fn to_string(&self) -> String {
        match self {
            ButtonType::Resume => "Resume".to_string(),
            ButtonType::Restart => "Restart".to_string(),
        }
    }
}
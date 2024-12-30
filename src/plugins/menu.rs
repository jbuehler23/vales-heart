use bevy::prelude::*;

use crate::{components::ui::MenuData, resources::GameState, systems::{input::handle_escape_menu, ui::{cleanup_menu, handle_pause_menu, pause_menu_ui}}};



pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuData>()
            .add_systems(Update, 
                handle_escape_menu
                    .run_if(not(in_state(GameState::ClassSelection)))
            )
            .add_systems(OnEnter(GameState::Paused), pause_menu_ui)
            .add_systems(Update, 
                handle_pause_menu
                    .run_if(in_state(GameState::Paused))
            )
            .add_systems(OnExit(GameState::Paused), cleanup_menu);
    }
}
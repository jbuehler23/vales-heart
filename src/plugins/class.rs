use bevy::prelude::*;
use bevy_egui::EguiContexts;
use crate::{
    components::{class::SelectedClass, ui::MenuData}, resources::GameState, systems::{class::spawn_selected_player, ui::{class_selection_ui, cleanup_menu, handle_class_selection}}
};

pub struct ClassPlugin;

impl Plugin for ClassPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuData>()
            .init_resource::<SelectedClass>()
            .add_systems(Startup, setup_initial_state)
            .add_systems(OnEnter(GameState::ClassSelection), class_selection_ui)
            .add_systems(Update, handle_class_selection.run_if(in_state(GameState::ClassSelection)))
            .add_systems(OnExit(GameState::ClassSelection), cleanup_menu)
            .add_systems(OnEnter(GameState::Playing), spawn_selected_player);
    }
}

fn setup_initial_state(
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("Setting initial game state to ClassSelection");
    next_state.set(GameState::ClassSelection);
}

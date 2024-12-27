use bevy::prelude::*;
use bevy_egui::EguiContexts;
use crate::{
    resources::GameState,
    systems::ui::{class_selection_ui, handle_class_selection},
};

pub struct ClassPlugin;

impl Plugin for ClassPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize game and force ClassSelection state
            .add_systems(Startup, setup_initial_state)
            // Set up UI when entering ClassSelection
            .add_systems(OnEnter(GameState::ClassSelection), class_selection_ui)
            .add_systems(Update, handle_class_selection);
            // // Handle class selection while preventing other state changes
            // .add_systems(Update, (
            //     prevent_state_skip).run_if(in_state(GameState::ClassSelection)));
    }
}

fn setup_initial_state(
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("Setting initial game state to ClassSelection");
    next_state.set(GameState::ClassSelection);
}

// Prevent accidental state transitions
// fn prevent_state_skip(
//     current_state: Res<State<GameState>>,
//     mut next_state: ResMut<NextState<GameState>>,
// ) {
//     if current_state.get() == &GameState::ClassSelection {
//         // If someone tries to set Playing state without class selection
//         if next_state == &GameState::Playing {
//             info!("Preventing premature transition to Playing state");
//             next_state.set(GameState::ClassSelection);
//         }
//     }
// }
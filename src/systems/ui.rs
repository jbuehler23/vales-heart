use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::components::class::{ClassType, PlayerClass};
use crate::resources::GameState;
use crate::systems::class::spawn_player_with_class;

pub fn class_selection_ui(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("Showing class selection UI");
    egui::Window::new("Class Selection")
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Choose Your Class");
            ui.add_space(8.0);

            let button_size = egui::vec2(160.0, 40.0);

            // Warrior
            ui.horizontal(|ui| {
                if ui.add_sized(button_size, egui::Button::new("Warrior")).clicked() {
                    spawn_player_with_class(commands.reborrow(), ClassType::Warrior);
                    next_state.set(GameState::Playing);
                }
                ui.label("Strong melee fighter with high health");
            });


            // Archer
            ui.horizontal(|ui| {
                if ui.add_sized(button_size, egui::Button::new("Archer")).clicked() {
                    spawn_player_with_class(commands.reborrow(), ClassType::Archer);
                    next_state.set(GameState::Playing);
                }
                ui.label("Skilled ranged fighter with high mobility");
            });

            // Mage
            ui.horizontal(|ui| {
                if ui.add_sized(button_size, egui::Button::new("Mage")).clicked() {
                    spawn_player_with_class(commands.reborrow(), ClassType::Mage);
                    next_state.set(GameState::Playing);
                }
                ui.label("Powerful spellcaster with high damage");
            });
        });
    }
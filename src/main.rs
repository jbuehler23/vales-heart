use bevy::{prelude::*, render::RenderPlugin, state::app::StatesPlugin, window::WindowResolution};



mod components;
mod plugins;
mod resources;
mod systems;
mod utils;

use plugins::PlayerPlugin;
use resources::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    // mode: bevy::window::WindowMode::Fullscreen,
                    resizable: true,
                    focused: true,
                    resolution: WindowResolution::new(800.0, 600.0),
                    name: Some("Vale's Heart".to_string()),
                    ..default()
                }),
                ..default()
            }),)
        // Game plugins
        .add_plugins(PlayerPlugin)
        // .add_plugin(WorldPlugin)
        // .add_plugin(CombatPlugin)
        // .add_plugin(InventoryPlugin)
        // .add_plugin(DialoguePlugin)
        // Add game state
        .insert_state(GameState::Loading)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
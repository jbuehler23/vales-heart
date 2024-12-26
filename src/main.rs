use bevy::{prelude::*, render::RenderPlugin, state::app::StatesPlugin, window::WindowResolution};



mod components;
mod plugins;
mod resources;
mod systems;
mod utils;

use bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::CollisionEvent;
use components::weapon::Weapon;
use plugins::{combat::CombatPlugin, physics::PhysicsPlugin, weapon::WeaponPlugin, player::PlayerPlugin};
use resources::GameState;
use systems::ui::class_selection_ui;

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
        // .add_plugin(WorldPlugin)
        // .add_plugin(CombatPlugin)
        // .add_plugin(InventoryPlugin)
        // .add_plugin(DialoguePlugin)
        // Add game state
        .add_plugins(EguiPlugin)
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::ClassSelection), class_selection_ui)
        .add_systems(Startup, setup)
        .add_systems(Update, log_collision_events)
        .add_plugins((
            PlayerPlugin,
            PhysicsPlugin,
            CombatPlugin,
            WeaponPlugin,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn log_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                info!("Collision started between entities: {:?} and {:?}", entity1, entity2);
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {
                info!("Collision stopped between entities: {:?} and {:?}", entity1, entity2);
            }
        }
    }
}
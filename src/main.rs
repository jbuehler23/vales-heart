use bevy::{prelude::*, render::RenderPlugin, state::app::StatesPlugin, window::WindowResolution};



mod components;
mod plugins;
mod resources;
mod systems;
mod utils;

use bevy_rapier2d::prelude::CollisionEvent;
use components::weapon::WeaponItem;
use plugins::{class::ClassPlugin, combat::CombatPlugin, inventory::InventoryPlugin, menu::MenuPlugin, physics::PhysicsPlugin, player::PlayerPlugin, weapon::WeaponPlugin};
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
        // Add camera and UI setup in Startup schedule
        .add_systems(Startup, setup)
        // Initialize game state
        .init_state::<GameState>()
        // Game plugins
        .add_plugins((
            ClassPlugin,
            PlayerPlugin,
            WeaponPlugin,
            PhysicsPlugin,
            CombatPlugin,
            MenuPlugin,
            InventoryPlugin,
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
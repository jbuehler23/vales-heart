use bevy::prelude::*;
use crate::{
    components::{assets::GameAssets, class::SelectedClass, ui::MenuData}, resources::GameState, systems::{animation::update_walk_animation, class::spawn_selected_player, ui::{class_selection_ui, cleanup_menu, handle_class_selection}}
};

const BASE_WARRIOR_PATH: &str = "sprites/characters/warrior/";
const BASE_ARCHER_PATH: &str = "sprites/characters/archer/";
const BASE_MAGE_PATH: &str = "sprites/characters/mage/";

pub struct ClassPlugin;

impl Plugin for ClassPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuData>()
            .init_resource::<SelectedClass>()
            .init_resource::<GameAssets>()
            .add_systems(Startup, (setup_initial_state, load_assets))
            .add_systems(OnEnter(GameState::ClassSelection), class_selection_ui)
            .add_systems(Update, 
                handle_class_selection
                    .run_if(in_state(GameState::ClassSelection))
            )
            .add_systems(OnExit(GameState::ClassSelection), cleanup_menu)
            .add_systems(OnEnter(GameState::Playing), (spawn_selected_player, update_walk_animation));
    }
}

fn setup_initial_state(
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("Setting initial game state to ClassSelection");
    next_state.set(GameState::ClassSelection);
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let game_assets = GameAssets {
        warrior_walk: vec![
            asset_server.load(BASE_WARRIOR_PATH.to_string() + "ElvenKnight_Walk_1.png"),
            asset_server.load(BASE_WARRIOR_PATH.to_string() + "ElvenKnight_Walk_2.png"),
            asset_server.load(BASE_WARRIOR_PATH.to_string() + "ElvenKnight_Walk_3.png"),
            asset_server.load(BASE_WARRIOR_PATH.to_string() + "ElvenKnight_Walk_4.png"),
        ],
        archer_walk: vec![
            asset_server.load(BASE_ARCHER_PATH.to_string() + "Ranger_Walk_1.png"),
            asset_server.load(BASE_ARCHER_PATH.to_string() + "Ranger_Walk_2.png"),
            asset_server.load(BASE_ARCHER_PATH.to_string() + "Ranger_Walk_3.png"),
            asset_server.load(BASE_ARCHER_PATH.to_string() + "Ranger_Walk_4.png"),
        ],
        mage_walk: vec![
            asset_server.load(BASE_MAGE_PATH.to_string() + "Wizard_Idle + Walk_1.png"),
            asset_server.load(BASE_MAGE_PATH.to_string() + "Wizard_Idle + Walk_2.png"),
            asset_server.load(BASE_MAGE_PATH.to_string() + "Wizard_Idle + Walk_3.png"),
            asset_server.load(BASE_MAGE_PATH.to_string() + "Wizard_Idle + Walk_4.png"),
        ],
    };
    
    commands.insert_resource(game_assets);
}

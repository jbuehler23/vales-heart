use bevy::{app::{App, Plugin, Startup, Update}, asset::AssetServer, prelude::{Commands, Res, ResMut}};

use crate::{components::assets::GameAssets, systems::animation::animation_system};

pub struct AssetLoadingPlugin;

const BASE_WARRIOR_PATH: &str = "sprites/characters/warrior/";
const BASE_ARCHER_PATH: &str = "sprites/characters/archer/";
const BASE_MAGE_PATH: &str = "sprites/characters/mage/";

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameAssets>()
            .add_systems(Startup, load_assets)
            .add_systems(Update, animation_system);
    }
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
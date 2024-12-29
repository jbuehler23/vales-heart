use bevy::prelude::*;
use crate::{components::player::Player, resources::GameState, systems::weapon::*};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(
        //     Update,
        //     spawn_weapon_for_player
        //         .run_if(in_state(GameState::Playing))
        //         .run_if(any_with_component::<Player>)
        // );
    }
}
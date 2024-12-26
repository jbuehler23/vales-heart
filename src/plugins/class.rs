use bevy::prelude::*;
use crate::{
    resources::GameState, systems::class::handle_class_selection,
};

pub struct ClassPlugin;

impl Plugin for ClassPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, 
                handle_class_selection
                    .run_if(in_state(GameState::ClassSelection))
            );
    }
}
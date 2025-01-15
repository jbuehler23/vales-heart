use bevy::{asset::Handle, image::Image, prelude::Resource};
#[derive(Resource)]

pub struct GameAssets {
    pub warrior_walk: Vec<Handle<Image>>,
    pub archer_walk: Vec<Handle<Image>>,
    pub mage_walk: Vec<Handle<Image>>,
}

impl Default for GameAssets {
    fn default() -> Self {
        Self {
            warrior_walk: Vec::new(),
            archer_walk: Vec::new(),
            mage_walk: Vec::new(),
        }
    }
}

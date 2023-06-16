use bevy::prelude::*;

pub mod game_interface;


#[derive(Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub textures: HashMap<&'static str, Handle<Image>>
}
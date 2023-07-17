use std::collections::HashMap;

use bevy::prelude::*;



#[derive(Resource)]
pub struct GraphicsAssets {
    pub logo: Handle<Image>,
    pub ascii_sheet: Handle<TextureAtlas>,
    pub font: Handle<Font>,
    pub textures: HashMap<&'static str, Handle<Image>>,
    pub map_textures: HashMap<&'static str, Handle<Image>>,
    pub map_items: HashMap<&'static str, Handle<Image>>
}
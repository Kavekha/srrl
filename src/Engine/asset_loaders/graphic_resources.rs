use std::collections::HashMap;

use bevy::prelude::*;



#[derive(Resource, Debug, Default)]
pub struct GraphicsAssets {
    pub logo: Handle<Image>,
    pub logo_layout: Handle<TextureAtlasLayout>,
    pub font: Handle<Font>,
    pub textures: HashMap<&'static str, Handle<Image>>,
    pub map_textures: HashMap<&'static str, Handle<Image>>,
    pub map_items: HashMap<&'static str, Handle<Image>>
}
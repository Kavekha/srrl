use std::collections::HashMap;

use bevy::prelude::*;



#[derive(Resource, Debug, Default)]
pub struct GraphicsAssets {
    //pub logo: Handle<Image>,
    //pub logo_layout: Handle<TextureAtlasLayout>,
    pub images: HashMap<&'static str, Handle<Image>>,
    pub font: Handle<Font>,
    pub textures: HashMap<&'static str, Handle<Image>>,
    pub map_textures: HashMap<&'static str, Handle<Image>>,
    pub map_items: HashMap<&'static str, Handle<Image>>,
    pub effects: HashMap<&'static str, Handle<Image>>,
    pub cursors: HashMap<&'static str, Handle<Image>>,
}
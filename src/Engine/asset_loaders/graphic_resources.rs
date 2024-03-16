use std::collections::HashMap;

use bevy::prelude::*;



#[derive(Resource, Debug, Default)]
pub struct GraphicsAssets {
    pub logo: Handle<Image>,
    pub ascii_sheet_layout: Handle<TextureAtlasLayout>,
    pub ascii_sheet_img: Handle<Image>,
    pub font: Handle<Font>,
    pub textures: HashMap<&'static str, Handle<Image>>,
    pub map_textures: HashMap<&'static str, Handle<Image>>,
    pub map_items: HashMap<&'static str, Handle<Image>>
}
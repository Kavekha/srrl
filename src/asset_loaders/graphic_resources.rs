use std::collections::HashMap;

use bevy::prelude::*;


#[derive(Default, Resource)]
pub struct AssetList (pub Vec<HandleUntyped>);

#[derive(Resource)]
pub struct GraphicsAssets {
    pub ascii_sheet: Handle<TextureAtlas>,
    pub font: Handle<Font>,
    pub textures: HashMap<&'static str, Handle<Image>>,
    pub map_textures: HashMap<&'static str, Handle<Image>>,
}
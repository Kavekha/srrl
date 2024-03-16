use bevy::prelude::*;

use self::audios::GameAudioPlugin;
use self::save_load_system::SaveLoadPlugin;
use self::asset_loaders::AssetsPlugin;
use self::render::GraphicsPlugin;

pub mod save_load_system;
pub mod asset_loaders;
pub mod states;
pub mod render;
pub mod audios;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AssetsPlugin)        
        .add_plugins(GameAudioPlugin)        
        .add_plugins(GraphicsPlugin)     
            .add_plugins(SaveLoadPlugin);
    }
}
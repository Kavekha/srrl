// Ici on charge Engine.
// Assets : Recupère les img & sons necessaires.
// Audio : Gère les sons & musiques.
// Graphismes: Gère le rendu graphique.
// SaveLoad : Gère la sauvegarde.

use bevy::prelude::*;

use self::audios::GameAudioPlugin;
use self::asset_loaders::AssetsPlugin;
use self::render::GraphicsPlugin;

pub mod save_load_system;
pub mod asset_loaders;
pub mod render;
pub mod audios;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AssetsPlugin)        
            .add_plugins(GameAudioPlugin)        
            .add_plugins(GraphicsPlugin)   
            ;
    }
}
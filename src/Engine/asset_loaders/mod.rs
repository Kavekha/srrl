// Gère les ressources.

use std::collections::HashMap;

use bevy::prelude::*;

pub mod graphic_resources;
pub mod audio_resources;

pub use graphic_resources::GraphicsAssets;
pub use audio_resources::AudioAssets;

const IMAGES: [&str; 1] = ["shadowrun_title_alone"];
const FONT_PATH: &str = "fonts/PressStart2P-vaV7.ttf";
const TEXTURES: [&str; 3] = ["human", "ghoul", "blood"];
const SEWERS_TILES_TEXTURES: [&str; 17] = [
    "floor", "wall_0","wall_1","wall_2","wall_3","wall_4","wall_5","wall_6","wall_7","wall_8",
    "wall_9","wall_10","wall_11","wall_12","wall_13","wall_14","wall_15"];
const SEWERS_ITEMS: [&str;1] = ["exit"];
const MUSICS: [&str;5] = ["main_menu", "combat", "gamemap", "gameover", "victory"];
const SOUNDS: [&str;3] = ["hit_punch_1", "death_scream", "hit_air_1"];//["air_hit", "fast_blow", "impact_blow", "impact_strong_punch", "soft_quick_punch"];


pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GraphicsAssets>()
            .init_resource::<AudioAssets>() 
            .add_systems(PreStartup, load_assets)
            ;
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Images
    let mut images = HashMap::new();
    for image_name in IMAGES {
        let handle:Handle<Image> = asset_server.load(format!("images/{}.png", image_name));
        println!("Debug: image handle is : {:?} for {}", handle.clone(), image_name);
        images.insert(image_name, handle);
    }

    // Font
    let font_handle:Handle<Font> = asset_server.load(FONT_PATH);

    // Sprites
    let mut textures = HashMap::new();
    for name in TEXTURES {
        let handle:Handle<Image> = asset_server.load(format!("characters/{}.png", name));
        textures.insert(name, handle);
    }

    // Sewer Map textures
    let mut sewer_textures = HashMap::new();
    for name in SEWERS_TILES_TEXTURES {
        let handle:Handle<Image> = asset_server.load(format!("tiles/sewers_{}.png", name));
         sewer_textures.insert(name, handle);
    }

    // Sewer items
    let mut sewer_items = HashMap::new();
    for name in SEWERS_ITEMS {
        let handle:Handle<Image> = asset_server.load(format!("map_items/sewers_{}.png", name));
         sewer_items.insert(name, handle);
    }

    // Musics
    let mut musics = HashMap::new();
    for name in MUSICS {
        let handle:Handle<AudioSource> = asset_server.load(format!("audios/{}.ogg", name));
        musics.insert(name, handle);
    }

    // Sounds
    let mut sounds = HashMap::new();
    for name in SOUNDS {
        let handle:Handle<AudioSource> = asset_server.load(format!("sounds/{}.ogg", name));
        sounds.insert(name, handle);
    }

    commands.insert_resource(
        GraphicsAssets { 
            images: images,
            font: font_handle,
            textures: textures,
            map_textures: sewer_textures,
            map_items: sewer_items,
        }
    );

    commands.insert_resource(
        AudioAssets {
            musics: musics,
            sounds:sounds
        }
    );

    println!("INFO: Assets loaded");    
}

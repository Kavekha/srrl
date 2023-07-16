use std::collections::HashMap;

use bevy::{prelude::*, asset::LoadState};

mod ascii;
mod graphic_resources;
mod audio_resources;

pub use ascii::{spawn_ascii_text, spawn_nine_slice, NineSliceIndices};
pub use graphic_resources::GraphicsAssets;
pub use audio_resources::AudioAssets;

use crate::states::AppState;


const ATLAS_PATH: &str = "ascii.png";
const FONT_PATH: &str = "fonts/PressStart2P-vaV7.ttf";
const TEXTURES: [&str; 2] = ["human", "ghoul"];
const SEWERS_TILES_TEXTURES: [&str; 17] = [
    "floor", "wall_0","wall_1","wall_2","wall_3","wall_4","wall_5","wall_6","wall_7","wall_8",
    "wall_9","wall_10","wall_11","wall_12","wall_13","wall_14","wall_15"];
const SEWERS_ITEMS: [&str;1] = ["exit"];

const MUSICS: [&str;5] = ["main_menu", "combat", "gamemap", "gameover", "victory"];

#[derive(Default, Resource)]
pub struct AssetList (pub Vec<HandleUntyped>);


pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AssetList>()
            
            .insert_resource(NineSliceIndices{
                center: 2 * 16,
                upper_left_index: 13 * 16 + 10,
                upper_right_index: 11 * 16 + 15,
                lower_left_index: 12 * 16,
                lower_right_index: 13 * 16 + 9,
                horizontal_index: 12 * 16 + 4,
                vertical_index: 11 * 16 + 3,
            })

            //.add_systems(OnEnter(AppState::AssetLoader), check_asset_loading)
            .add_systems(PreStartup, load_assets)
            .add_systems(Update, check_asset_loading.run_if(in_state(AppState::AssetLoader)))
            ;
    }
}
// https://stackoverflow.com/questions/75352459/access-bevy-asset-right-after-loading-via-assetserver
pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<AssetList>,
 ) {
 
    // Ascii
    let texture = asset_server.load(ATLAS_PATH);
    asset_list.0.push(texture.clone_untyped());
    let atlas = TextureAtlas::from_grid(
        texture,
        Vec2::splat(9.0),
        16,
        16,
        Some(Vec2::splat(2.0)),
        None);
    let atlas_handle:Handle<TextureAtlas> = texture_atlases.add(atlas);

    // Font
    let font_handle:Handle<Font> = asset_server.load(FONT_PATH);
    asset_list.0.push(font_handle.clone_untyped());   

    // Sprites
    let mut textures = HashMap::new();
    for name in TEXTURES {
        let handle:Handle<Image> = asset_server.load(format!("characters/{}.png", name));
        asset_list.0.push(handle.clone_untyped());
        textures.insert(name, handle);
    }

    // Sewer Map textures
    let mut sewer_textures = HashMap::new();
    for name in SEWERS_TILES_TEXTURES {
        let handle:Handle<Image> = asset_server.load(format!("tiles/sewers_{}.png", name));
        asset_list.0.push(handle.clone_untyped());
        sewer_textures.insert(name, handle);
    }

    // Sewer items
    let mut sewer_items = HashMap::new();
    for name in SEWERS_ITEMS {
        let handle:Handle<Image> = asset_server.load(format!("map_items/sewers_{}.png", name));
        asset_list.0.push(handle.clone_untyped());
        sewer_items.insert(name, handle);
    }

    // Musics
    let mut musics = HashMap::new();
    for name in MUSICS {
        let handle:Handle<AudioSource> = asset_server.load(format!("audios/{}.ogg", name));
        asset_list.0.push(handle.clone_untyped());
        musics.insert(name, handle);
    }
 
    commands.insert_resource(
        GraphicsAssets { 
            ascii_sheet: atlas_handle,
            font: font_handle,
            textures: textures,
            map_textures: sewer_textures,
            map_items: sewer_items,
        }
    );

    commands.insert_resource(
        AudioAssets {
            musics: musics
        }
    )
}



fn check_asset_loading(
    asset_server: Res<AssetServer>,
    asset_list: Res<AssetList>,
    mut next_state: ResMut<NextState<AppState>>
){
    match asset_server.get_group_load_state(
        asset_list.0.iter().map(|a| a.id())
    ) {
        LoadState::Loaded => {
            next_state.set(AppState::MainMenu);
        },
        LoadState::Failed => {
            error!("asset loading error");
        },
        _ => {}
    };
}


use bevy::{prelude::*, asset::LoadState};

mod ascii;
mod graphic_resources;

pub use ascii::{spawn_ascii_text, spawn_nine_slice, NineSliceIndices};
pub use graphic_resources::{AssetList, GraphicsAssets};

use crate::states::AppState;



const ATLAS_PATH: &str = "ascii.png";
const FONT_PATH: &str = "fonts/PressStart2P-vaV7.ttf";


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
    asset_list.0.push(font_handle.clone_untyped());   //TODO: la liste permets de checker si toutes les resources sont load.

 
    commands.insert_resource(
        GraphicsAssets { 
            ascii_sheet: atlas_handle,
            font: font_handle
        }
    );
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


use bevy::prelude::*;

use crate::{engine::asset_loaders::GraphicsAssets, globals::{ CURSOR, ORDER_CURSOR}};

use super::components::GameCursorRender;


pub fn spawn_game_cursor(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    graph_assets: Res<GraphicsAssets>
){
    println!("Spawning Cursor");
    commands.spawn(SpriteBundle {
                texture: graph_assets.cursors["cursor_moving"].clone(),//asset_server.load(CURSOR),    
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, ORDER_CURSOR),  
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                ..default()            
            })
            .insert(Name::new("Cursor"))
            .insert(GameCursorRender)
            ;
}



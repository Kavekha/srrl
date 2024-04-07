use bevy::prelude::*;

use crate::globals::{ CURSOR, ORDER_CURSOR};

use super::components::GameCursorRender;


pub fn spawn_game_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    println!("Spawning Cursor");
    commands.spawn(SpriteBundle {
                texture: asset_server.load(CURSOR),    
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



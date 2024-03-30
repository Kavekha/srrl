use bevy::prelude::*;

use crate::{
    globals::{ POSITION_TOLERANCE, CURSOR_SPEED, SPEED_MULTIPLIER, CURSOR, ORDER_CURSOR},
    game::player::Cursor
};

use super::{components::GameCursorRender, get_world_position};


pub fn spawn_game_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    println!("Spawning Cursor");
    commands.spawn(GameCursorRender)
            .insert(SpriteBundle {
                texture: asset_server.load(CURSOR),    
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, ORDER_CURSOR),  
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                ..default()            
            })
            .insert(Name::new("Cursor"))
            ;
}



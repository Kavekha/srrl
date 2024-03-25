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


pub fn update_game_cursor(
    mut query_game_cursor: Query<(&GameCursorRender, &mut Transform)>,
    cursor_position: Res<Cursor>,
    time: Res<Time>
){
    for (_game_cursor, mut transform, ) in query_game_cursor.iter_mut(){
        let grid_position = &cursor_position.grid_position;
        let position = get_world_position(grid_position);


        //let position = &cursor_position.world_position;

        let target = Vec3::new(position.0, position.1, ORDER_CURSOR);
        let destination = (target - transform.translation).length();  
        //println!("Cursor update: target is {:?}, transform is : {:?}, destination is : {:?}", target, transform.translation, destination);
        
        if destination > POSITION_TOLERANCE {
            transform.translation = transform.translation.lerp(
                target,
                CURSOR_SPEED * SPEED_MULTIPLIER * time.delta_seconds()
            );
        } else {
            transform.translation = target;
        }
    }
}
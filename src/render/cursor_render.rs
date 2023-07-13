use bevy::prelude::*;

use crate::{
    globals::{CURSOR_FRONT, POSITION_TOLERANCE, CURSOR_SPEED, SPEED_MULTIPLIER, CURSOR_BACK},
    game::player::Cursor
};

use super::components::GameCursorRender;


pub fn spawn_game_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands.spawn(GameCursorRender)
            .insert(SpriteBundle {
                texture: asset_server.load(CURSOR_FRONT),    
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                ..default()            
            })
            .insert(Name::new("Cursor"))
            ;

    commands.spawn(GameCursorRender)
            .insert(SpriteBundle{
                texture: asset_server.load(CURSOR_BACK),    
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::splat(1.0),
                    ..default()
                },
                ..default()    
            })
            .insert(Name::new("Back cursor"))
            ;    
}


pub fn update_game_cursor(
    mut query_game_cursor: Query<(&GameCursorRender, &mut Transform)>,
    cursor_position: Res<Cursor>,
    time: Res<Time>
){
    for (_game_cursor, mut transform, ) in query_game_cursor.iter_mut(){
        let position = &cursor_position.world_position;

        let target = Vec3::new(position.x, position.y, 0.0);
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
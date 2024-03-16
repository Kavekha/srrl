// Gère le Rendu.

use bevy::prelude::*;

pub mod tilemap_render;
pub mod pieces_render;
pub mod components;
pub mod cursor_render;


use self::{
    tilemap_render::spawn_map_render,
    pieces_render::{spawn_piece_renderer, path_animator_update, spawn_exit_render}, //melee_animation
    cursor_render::{spawn_game_cursor, update_game_cursor},
};

use crate::{
    globals::STANDARD_TILE_SIZE, 
    engine::states::GameState, vectors::Vector2Int, game::combat::CombatSet,
};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_event::<GraphicsWaitEvent>()
    
            .add_systems(OnEnter(GameState::GameMap), spawn_map_render)                  
            .add_systems(OnEnter(GameState::GameMap), spawn_piece_renderer)
            .add_systems(OnEnter(GameState::GameMap), spawn_game_cursor)     
            .add_systems(OnEnter(GameState::GameMap), spawn_exit_render)    

            //.add_systems(Update, (walk_animation, path_animator_update, melee_animation).in_set(TurnSet::Animation))
            .add_systems(Update, (path_animator_update).in_set(CombatSet::Animation))   //melee_animation
            .add_systems(Update, update_game_cursor.in_set(CombatSet::Animation))     

            // La première camera.
            .add_systems(Startup, spawn_camera)    
            ;
    }
}


fn spawn_camera(mut commands: Commands) {
    println!("Camera is spawned");
    commands.spawn(Camera2dBundle::default()); //DEBUG

    // Before 0.13. 
    /*      
    let camera_bundle = Camera2dBundle {
        projection: OrthographicProjection{
            far: 100.0,
            near: -100.0,
            scaling_mode: ScalingMode::WindowSize(500.0), //(1.0 * BASE_SCREEN_SCALE),    //WindowSize(500.0),   // Pixels = world unit
            ..default()
        },
        ..default()
    };
    commands.spawn(camera_bundle);   
    */ 
}


/* 
#[derive(Event)]
pub struct GraphicsWaitEvent;
*/

// TODO : Changer de place?

pub fn get_world_position(
    v: &Vector2Int
) -> (f32, f32) {
        // REMEMBER : Y in bevy2d = Negative when going down!
        let x = v.x * STANDARD_TILE_SIZE;
        let y = v.y  * STANDARD_TILE_SIZE;

        //println!("GetWorldPosition : {:?} gives {:?}. World position get grid position : {:?}", (v.x, v.y), (iso_x, iso_y), get_grid_position(iso_x as f32, 0.0 - iso_y as f32));

        (x as f32,
        0.0 - y as f32)     // REMEMBER : Y in bevy2d = Negative when going down!

}

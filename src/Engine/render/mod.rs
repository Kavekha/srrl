// Gère le Rendu.

use bevy::prelude::*;

pub mod components;
mod tilemap_render;
mod pieces_render;
mod cursor_render;


use crate::game::states::GameState;

use self::{
    tilemap_render::spawn_map_render,
    pieces_render::{spawn_piece_renderer, spawn_exit_render}, //melee_animation
    cursor_render::spawn_game_cursor,
};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app    
            .add_systems(OnEnter(GameState::Initialise), spawn_map_render)                  
            .add_systems(OnEnter(GameState::Initialise), spawn_piece_renderer)
            .add_systems(OnEnter(GameState::Initialise), spawn_game_cursor)     
            .add_systems(OnEnter(GameState::Initialise), spawn_exit_render)    
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





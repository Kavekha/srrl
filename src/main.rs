//#![windows_subsystem = "windows"]     // Empeche de voir le terminal dans VS Code.... -_-
#![allow(clippy::redundant_field_names)]
use bevy::{
    prelude::*, window::PresentMode::Fifo   //, render::camera::ScalingMode
};

mod engine;
mod map_builders;   //mod
mod game;           //mod
mod globals;
mod vectors;

use game::GamePlugin;
use engine::EnginePlugin;
use engine::states::{AppState, GameState, EngineState};
use globals::{HEIGHT, RESOLUTION, CLEAR};




fn main() {
    println!("App launched");

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ShadowRun:RL".to_string(),
                        resolution: (HEIGHT * RESOLUTION, HEIGHT).into(),
                        present_mode: Fifo, //AutoVsync,
                        resizable: false, 
                        ..Default::default()
                }),
                ..default()
                })
                .set(
                    ImagePlugin::default_nearest()
                )
        )

        // Engine 
        .add_plugins(EnginePlugin)

        //.add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        //.add_plugins(InGameMenuPlugin)
        
        .init_state::<AppState>()
        .init_state::<GameState>()  
        .init_state::<EngineState>()

        .add_systems(Startup, spawn_camera)
        .run(); 
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

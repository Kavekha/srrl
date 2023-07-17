//#![windows_subsystem = "windows"]     // Empeche de voir le terminal dans VS Code.... -_-
#![allow(clippy::redundant_field_names)]
use bevy::{
    prelude::*, 
    render::camera::ScalingMode,
    window::PresentMode::Fifo,
};


mod audios;
mod menus;
mod map_builders;   //mod
mod game;           //mod
mod save_load_system;
mod ecs_elements;
mod globals;
mod render;
mod states;
mod vectors;
mod asset_loaders;

//pub use render::GraphicsWaitEvent;

use menus::mainmenu::MainMenuPlugin;
use audios::GameAudioPlugin;
use game::GamePlugin;
use save_load_system::SaveLoadPlugin;
use asset_loaders::AssetsPlugin;

use states::{AppState, GameState, EngineState};
use globals::{HEIGHT, RESOLUTION, CLEAR, BASE_SCREEN_SCALE};



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
 
        .add_plugins(MainMenuPlugin)
        .add_plugins(GameAudioPlugin)     
        .add_plugins(GamePlugin)
        .add_plugins(SaveLoadPlugin)
        .add_plugins(AssetsPlugin)

        .add_state::<AppState>()
        .add_state::<GameState>()  
        .add_state::<EngineState>()

        .add_systems(Startup, spawn_camera)
        .run(); 
}


fn spawn_camera(mut commands: Commands) {
    let camera_bundle = Camera2dBundle {
        projection: OrthographicProjection{
            scaling_mode: ScalingMode::WindowSize(1.0 * BASE_SCREEN_SCALE),    //WindowSize(500.0),   // Pixels = world unit
            ..default()
        },
        ..default()
    };
    commands.spawn(camera_bundle);
}



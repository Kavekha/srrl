//#![windows_subsystem = "windows"]     // Empeche de voir le terminal dans VS Code.... -_-
#![allow(clippy::redundant_field_names)]
use bevy::{
    prelude::*, 
    render::camera::ScalingMode,
    window::PresentMode::Fifo,
};

mod ascii;
mod audio;
mod commons;
mod menus;
mod map_builders;   //mod
mod game;           //mod
mod save_load_system;
mod ecs_elements;
mod globals;
mod render;
mod states;
mod vectors;

pub use render::GraphicsWaitEvent;

use ascii::AsciiPlugin;
use menus::mainmenu::MainMenuPlugin;
use audio::GameAudioPlugin;
use game::GamePlugin;
use save_load_system::SaveLoadPlugin;
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

        .add_plugin(AsciiPlugin) 
        .add_plugin(MainMenuPlugin)
        .add_plugin(GameAudioPlugin)     
        .add_plugin(GamePlugin)
        .add_plugin(SaveLoadPlugin)

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


/* 
pub fn despawn_screen<T: Component>(
    to_despawn: Query<Entity, With<T>>, 
    mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
*/
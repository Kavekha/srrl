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

use ascii::AsciiPlugin;
use menus::mainmenu::MainMenuPlugin;
use audio::GameAudioPlugin;
use game::GameState;
use game::GamePlugin;
use save_load_system::SaveLoadPlugin;


pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 800.0;

pub const TILE_SIZE: f32 = 32.0;
pub const CHAR_SIZE: f32 = 16.0;  

const SHOW_MAPGEN_VISUALIZER : bool = false;    //DEBUG 




#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game
}




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
        .add_state::<AppState>()
        .add_state::<GameState>()   //TO MOVE elsewhere (game thingy)
        .add_systems(Startup, spawn_camera)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GameAudioPlugin)     
        .add_plugin(GamePlugin)
        .add_plugin(SaveLoadPlugin)
        .run(); 
}


fn spawn_camera(mut commands: Commands) {
    let camera_bundle = Camera2dBundle {
        projection: OrthographicProjection{
            scaling_mode: ScalingMode::WindowSize(1.0),    //WindowSize(500.0),   // Pixels = world unit
            ..default()
        },
        ..default()
    };
    commands.spawn(camera_bundle);
}


pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
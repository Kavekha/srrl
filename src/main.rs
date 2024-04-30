//#![windows_subsystem = "windows"]     // Empeche de voir le terminal dans VS Code.... -_-
#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, window::PresentMode::Fifo   //, render::camera::ScalingMode
};

#[macro_use]
extern crate lazy_static;

mod engine;
mod map_builders;
mod game;           
mod globals;
mod vectors;
mod menu_builders;
mod commons;
mod raws;
mod camera;

use game::GamePlugin;
use engine::EnginePlugin;
use game::states::GameState;   
use globals::{HEIGHT, RESOLUTION, CLEAR};

use crate::game::states::MenuState;




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

        .add_plugins(EnginePlugin)
        .add_plugins(GamePlugin)
        
        .init_state::<GameState>()
        .init_state::<MenuState>()


        .run(); 
}

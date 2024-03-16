// On charge ici les principaux modules:
// Engine
// Game


//#![windows_subsystem = "windows"]     // Empeche de voir le terminal dans VS Code.... -_-
#![allow(clippy::redundant_field_names)]
use bevy::{
    prelude::*, window::PresentMode::Fifo   //, render::camera::ScalingMode
};

mod engine;
mod map_builders;
mod game;           
mod globals;
mod vectors;

use game::GamePlugin;
use engine::EnginePlugin;
//use engine::states::{AppState, GameState, EngineState};
use engine::states::{GameState, EngineState};
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

        .add_plugins(EnginePlugin)
        .add_plugins(GamePlugin)

        // TODO : Deplacer dans Engine / Reduire le nombre.
        //.init_state::<AppState>()
        .init_state::<GameState>()  
        .init_state::<EngineState>()


        .run(); 
}



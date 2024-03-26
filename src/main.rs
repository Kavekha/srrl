// On charge ici les principaux modules:
// Engine
// Game


//#![windows_subsystem = "windows"]     // Empeche de voir le terminal dans VS Code.... -_-
#![allow(clippy::redundant_field_names)]
use bevy::{
    audio::Volume, prelude::*, window::PresentMode::Fifo   //, render::camera::ScalingMode
};

mod engine;
mod map_builders;
mod game;           
mod globals;
mod vectors;

use game::GamePlugin;
use engine::EnginePlugin;
use game::states::{GameState, EngineState};
use globals::{HEIGHT, RESOLUTION, CLEAR};

use crate::game::states::MenuState;




fn main() {
    println!("App launched");

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(AudioConfig {
            sound_active:false, sound_volume:Volume::new(5.0),
            music_active:false, music_volume:Volume::new(5.0)
        })
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
        .init_state::<EngineState>()
        .init_state::<MenuState>()


        .run(); 
}

#[derive(Resource)]
pub struct AudioConfig {
    pub sound_active: bool,
    pub sound_volume: Volume,
    pub music_active: bool,
    pub music_volume: Volume
}
#![allow(clippy::redundant_field_names)]
use bevy::{
    prelude::*, 
    render::camera::ScalingMode,
    window::PresentMode::Fifo,
};


pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1; 
pub const HEIGHT: f32 = 900.0;

mod player;
mod ascii;
mod tilemap;

use player::PlayerPlugin;
use ascii::AsciiPlugin;
use tilemap::TileMapPlugin;


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ShadowRun: POC".to_string(),
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
        .add_systems(Startup, spawn_camera)
        .add_plugin(TileMapPlugin)
        .add_plugin(PlayerPlugin)    
        .run();
}


fn spawn_camera(mut commands: Commands) {
    let camera_bundle = Camera2dBundle {
        projection: OrthographicProjection{
            scaling_mode: ScalingMode::WindowSize(500.0),
            ..default()
        },
        ..default()
    };
    commands.spawn(camera_bundle);
}
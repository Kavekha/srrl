use bevy::prelude::*;

mod components;

use crate::{
    AppState,
    GameState, asset_loaders::AudioAssets,
};

use self::components::CurrentMusic;


pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin{
    fn build(
        &self, app:&mut App
    ) {
        app
            //TODO: Something else than a function & system by music...
            //GameMap
            .add_systems(OnEnter(GameState::GameMap), setup_audio_gamemap)
            .add_systems(OnExit(GameState::GameMap), stop_music)
            //Victory
            .add_systems(OnEnter(GameState::VictoryScreen), setup_audio_victory)
            .add_systems(OnExit(GameState::VictoryScreen), stop_music)
            //Main Menu
            .add_systems(OnEnter(AppState::MainMenu), setup_audio_mainmenu)
            .add_systems(OnExit(AppState::MainMenu), stop_music)           
            //Death
            .add_systems(OnEnter(GameState::GameOverScreen), setup_audio_death)
            .add_systems(OnExit(GameState::GameOverScreen), stop_music)
            ;         
    }
}

//TODO : Refacto audio to avoid duplicate.
fn setup_audio_mainmenu(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    assets: Res<AudioAssets>
) {
    commands.spawn((
        AudioBundle {
            //source: asset_server.load("audios/Seattle-2050.ogg"),
            source: assets.musics["main_menu"].clone(),
            ..default()},
        CurrentMusic,
        ));
}

fn setup_audio_death(
    mut commands: Commands,
    assets: Res<AudioAssets>
) {
    commands.spawn((AudioBundle {
        source: assets.musics["gameover"].clone(),
        ..default()},
        CurrentMusic,
        ));
}

fn setup_audio_victory(
    mut commands: Commands,
    assets: Res<AudioAssets>
) {
    commands.spawn((AudioBundle {
        source: assets.musics["victory"].clone(),
        ..default()},
        CurrentMusic,
        ));
}

fn setup_audio_gamemap(
    mut commands: Commands,
    assets: Res<AudioAssets>
) {
    commands.spawn((
        AudioBundle {
            source: assets.musics["gamemap"].clone(),
        ..default()},
        CurrentMusic,
    ));
}


fn stop_music(
    // `AudioSink` will be inserted by Bevy when the audio starts playing
    query_music: Query<&AudioSink>  //, With<CurrentMusic>>,
) {
    println!("Stop Music: Start");
    for sink in query_music.iter() {
        sink.stop();
    };
    /* 
    if let Ok(sink) = query_music.get_single() {
        sink.stop();
        println!("Stop Music: Sink.Stop");
    };*/
}

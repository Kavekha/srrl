use bevy::prelude::*;

mod components;

use crate::{
    AppState,
    GameState,
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
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audios/Seattle-2050.ogg"),
            ..default()},
        CurrentMusic,
        ));
}

fn setup_audio_death(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((AudioBundle {
        source: asset_server.load("audios/Dead.ogg"),
        ..default()},
        CurrentMusic,
        ));
}

fn setup_audio_victory(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((AudioBundle {
        source: asset_server.load("audios/Ending.ogg"),
        ..default()},
        CurrentMusic,
        ));
}

fn setup_audio_gamemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        AudioBundle {
        source: asset_server.load("audios/Morgue.ogg"),
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

/* 

fn stop_music(
    audio_sinks: Res<Assets<AudioSink>>,
    music_controller: Res<MusicController>,
){
    if let Some(sink) = audio_sinks.get(&music_controller.0){
        sink.stop()
    }
}


pub fn setup_audio_death(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
){
    let music = audio.play_with_settings(
        asset_server.load("audios/Dead.ogg"),
        PlaybackSettings::LOOP,
    );
    let handle = audio_sinks.get_handle(music);
    commands.insert_resource(MusicController(handle));
}

pub fn setup_audio_victory(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
){
    let music = audio.play_with_settings(
        asset_server.load("audios/Ending.ogg"),
        PlaybackSettings::LOOP,
    );
    let handle = audio_sinks.get_handle(music);
    commands.insert_resource(MusicController(handle));
}

pub fn setup_audio_gamemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
){
    let music = audio.play_with_settings(
        asset_server.load("audios/Morgue.ogg"),
        PlaybackSettings::LOOP,
    );
    let handle = audio_sinks.get_handle(music);
    commands.insert_resource(MusicController(handle));
}

pub fn setup_audio_mainmenu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
){
    let music = audio.play_with_settings(
        asset_server.load("audios/Seattle-2050.ogg"),
        PlaybackSettings::LOOP,
    );
    let handle = audio_sinks.get_handle(music);
    commands.insert_resource(MusicController(handle));
}
*/
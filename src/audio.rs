use bevy::prelude::*;
use crate::{
    mainmenu::MenuState,
    GameState
};


pub struct GameAudioPlugin;

#[derive(Resource)]
struct MusicController(Handle<AudioSink>);


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
            .add_systems(OnEnter(MenuState::MainMenu), setup_audio_mainmenu)
            .add_systems(OnExit(MenuState::MainMenu), stop_music);           
    }
}


fn stop_music(
    audio_sinks: Res<Assets<AudioSink>>,
    music_controller: Res<MusicController>,
){
    if let Some(sink) = audio_sinks.get(&music_controller.0){
        sink.stop()
    }
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

#[warn(dead_code)]
fn pause_audio(
    audio_sinks: Res<Assets<AudioSink>>,
    music_controller: Res<MusicController>,
) {
    if let Some(sink) = audio_sinks.get(&music_controller.0) {
        if !sink.is_paused() {
            sink.pause()
        }
    }
}
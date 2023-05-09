use bevy::prelude::*;
use crate::mainmenu::MenuState;

pub struct GameAudioPlugin;

#[derive(Resource)]
struct MusicController(Handle<AudioSink>);


impl Plugin for GameAudioPlugin{
    fn build(
        &self, app:&mut App
    ) {
        //app.add_systems(Startup, load_audio)
        app.add_systems(OnEnter(MenuState::MainMenu), play_music);
    }
}

pub fn play_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("audios/Windless Slopes.ogg");
    audio.play(music);
}

/*
fn load_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
){
    let opening_handle = asset_server.load("assets/audios/01 Seattle 2050 (American).mp3");
    //let combat_handle = asset_server.load("assets/audios/09 Gunfight.mp3");
    //let bgm_handle = asset_server.load("assets/audios/04 Morgue.mp3");
    //let victory_handle = asset_server.load("assets/audios/16 Ending.mp3");
    //let dead_handle = asset_server.load("assets/audios/10 Dead.mp3");

    let handle = audio_sinks.get_handle(audio.play(opening_handle));
    commands.insert_resource(MusicController(handle));
} */
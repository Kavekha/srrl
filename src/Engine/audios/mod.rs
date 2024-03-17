// TODO : Comment gèrer ça autrement?

use bevy::prelude::*;

mod components;

use crate::{
    //AppState,
    GameState, engine::asset_loaders::AudioAssets,
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
            //Victory
            .add_systems(OnEnter(GameState::VictoryScreen), setup_audio_victory)
            //Main Menu
            //.add_systems(OnEnter(AppState::MainMenu), setup_audio_mainmenu)     
            //Death
            .add_systems(OnEnter(GameState::GameOverScreen), setup_audio_death)
            ;
        println!("INFO: Audioplugin loaded.");    
    }    
}

//TODO : Refacto audio to avoid duplicate.
fn setup_audio_mainmenu(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    assets: Res<AudioAssets>,
    query_music: Query<&AudioSink> 
) {
    println!("audio: setup audio mainmenu");
    stop_music(query_music);
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
    assets: Res<AudioAssets>,
    query_music: Query<&AudioSink> 
) {
    stop_music(query_music);
    commands.spawn((AudioBundle {
        source: assets.musics["gameover"].clone(),
        ..default()},
        CurrentMusic,
        ));
}

fn setup_audio_victory(
    mut commands: Commands,
    assets: Res<AudioAssets>,
    query_music: Query<&AudioSink> 
) {
    stop_music(query_music);
    commands.spawn((AudioBundle {
        source: assets.musics["victory"].clone(),
        ..default()},
        CurrentMusic,
        ));
}

fn setup_audio_gamemap(
    mut commands: Commands,
    assets: Res<AudioAssets>,
    query_music: Query<&AudioSink> 
) {
    stop_music(query_music);
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
}

/*
-- Envoyer de la musique:
fn test_music_event(mut ev_message: EventWriter<MessageEvent>) {ev_message.send(MessageEvent(Box::new(PlayMusicMessage{source:"main_menu".to_string()}))); } 
*/

use bevy::prelude::*;

pub mod components;

use crate::engine::asset_loaders::AudioAssets;

use self::components::CurrentMusic;

 
pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin{
    fn build(
        &self, app:&mut App
    ) {
        app
            .add_event::<MusicEvent>()   
            .add_systems(Update, handle_music_event.run_if(on_event::<MusicEvent>()))
            ;
        println!("INFO: Audioplugin loaded.");    
    }    
}

#[derive(Event)]
pub struct MusicEvent{
    pub source: String
}

fn handle_music_event(
    mut commands: Commands,
    assets: Res<AudioAssets>,
    query_music: Query<&AudioSink>,
    mut ev_music: EventReader<MusicEvent>,
) {
    stop_music(query_music);
    for event in ev_music.read() {
        println!("audio: setup audio handle: source is {}", event.source);
        commands.spawn((
            AudioBundle {
                //source: asset_server.load("audios/Seattle-2050.ogg"),
                source: assets.musics[event.source.as_str()].clone(),
                ..default()},
            CurrentMusic,
            ));
    }
}

pub fn stop_music(
    // `AudioSink` will be inserted by Bevy when the audio starts playing
    query_music: Query<&AudioSink>,  //, With<CurrentMusic>>,
) {
    println!("Stop Music: Start");
    for sink in query_music.iter() {
        sink.stop();
    };
}

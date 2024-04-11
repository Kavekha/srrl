/*
-- Envoyer de la musique:
fn test_music_event(mut ev_message: EventWriter<MessageEvent>) {ev_message.send(MessageEvent(Box::new(PlayMusicMessage{source:"main_menu".to_string()}))); } 
*/

use bevy::{audio::Volume, prelude::*};
use bevy::audio::PlaybackMode;

use crate::engine::asset_loaders::AudioAssets;
use crate::globals::DEFAULT_VOLUME;

use self::components::{CurrentMusic, CurrentSound};


mod components;
 
pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin{
    fn build(
        &self, app:&mut App
    ) {
        app
            .insert_resource(AudioConfig {
                sound_active:true, sound_volume:Volume::new(DEFAULT_VOLUME),
                music_active:true, music_volume:Volume::new(DEFAULT_VOLUME)
            })

            .add_event::<MusicEvent>()   
            .add_event::<SoundEvent>()   

            .add_systems(Update, handle_music_event.run_if(on_event::<MusicEvent>()))
            .add_systems(Update, handle_sound_event.run_if(on_event::<SoundEvent>()))
            ;
        println!("INFO: Audioplugin loaded.");    
    }    
}

#[derive(Resource)]
pub struct AudioConfig {
    pub sound_active: bool,
    pub sound_volume: Volume,
    pub music_active: bool,
    pub music_volume: Volume
}
#[derive(Resource, Clone)]
pub enum AudioType{
    Music,
    Sound
}

#[derive(Event)]
pub struct MusicEvent{
    pub source: String
}

#[derive(Event)]
pub struct SoundEvent{
    pub id: String
}

fn handle_sound_event(
    mut commands: Commands,
    assets: Res<AudioAssets>,
    _query_music: Query<&AudioSink>,
    mut ev_sound: EventReader<SoundEvent>,
    config: Res<AudioConfig>
) {
    let to_play = config.sound_active;
    if to_play == false { return };
    // Not working with ogg, wav in music or sound... ? trop court?
    for event in ev_sound.read() {
        println!("audio: sound is {}", event.id);
        let playback = PlaybackSettings{
            mode: PlaybackMode::Despawn,
            volume: config.sound_volume,
            ..default()
        };
        commands.spawn((
            AudioBundle {
                source: assets.sounds[event.id.as_str()].clone(),
                settings: playback,
                ..default()},
            CurrentSound,
            ));
        }

}

fn handle_music_event(
    mut commands: Commands,
    assets: Res<AudioAssets>,
    query_music: Query<&AudioSink>,
    mut ev_music: EventReader<MusicEvent>,
    config: Res<AudioConfig>
) {
    stop_music(query_music);
    let to_play = config.music_active;
    if to_play == false { return };
    for event in ev_music.read() {
        println!("audio: setup audio handle: source is {}", event.source);
        let playback = PlaybackSettings{
            mode: PlaybackMode::Loop,
            volume: config.music_volume, //Volume::new(10.0),
            ..default()
        };
        commands.spawn((
            AudioBundle {
                //source: asset_server.load("audios/Seattle-2050.ogg"),
                source: assets.musics[event.source.as_str()].clone(),
                settings: playback,
                ..default()},
            CurrentMusic,
            ));
    }
}

pub fn stop_music(
    // `AudioSink` will be inserted by Bevy when the audio starts playing
    query_music: Query<&AudioSink>,  //, With<CurrentMusic>>,
) {
    println!("Stop Music");
    for sink in query_music.iter() {
        sink.stop();
    };
}

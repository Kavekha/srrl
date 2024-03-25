// Gère les assets audios.

use std::collections::HashMap;

use bevy::prelude::*;


#[derive(Resource, Debug, Default)]
pub struct AudioAssets {
    pub musics: HashMap<&'static str, Handle<AudioSource>>,
    pub sounds: HashMap<&'static str, Handle<AudioSource>>
}
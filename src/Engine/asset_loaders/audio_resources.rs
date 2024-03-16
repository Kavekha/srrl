use std::collections::HashMap;

use bevy::prelude::*;


#[derive(Resource, Debug, Default)]
pub struct AudioAssets {
    pub musics: HashMap<&'static str, Handle<AudioSource>>,
}
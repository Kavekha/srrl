use std::collections::HashMap;

use bevy::prelude::*;


#[derive(Resource)]
pub struct AudioAssets {
    pub musics: HashMap<&'static str, Handle<AudioSource>>,
}
use bevy::prelude::*;

use super::map::Map;


#[derive(Resource)]
pub struct MapGenHistory{
    pub history: Vec<Map>,
    pub index: usize,
}
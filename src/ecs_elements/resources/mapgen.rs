use bevy::prelude::*;

use crate::map_builders::map::Map;


#[derive(Resource)]
pub struct MapGenHistory{
    pub history: Vec<Map>,
    pub index: usize,
}
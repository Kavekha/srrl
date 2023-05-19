use bevy::prelude::*;

pub mod map;
pub mod rectangle;
pub mod simple_map;
pub mod pathfinding;
pub mod commons;

use crate::{
    map_builders::{
        map::{Map},
        simple_map::SimpleMapBuilder,
    },
    map_builders::pathfinding::Position,
};


pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self) -> Vec<Position>;
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
    fn get_snapshot_history(&self) -> Vec<Map>;
    fn take_snapshot(&mut self);
}

pub fn random_builder() -> Box<dyn MapBuilder> {
    Box::new(SimpleMapBuilder::new())       //Return so no semicon... REMEMBER
}

#[derive(Resource)]
pub struct MapGenHistory{
    pub history: Vec<Map>,
    pub index: usize,
}
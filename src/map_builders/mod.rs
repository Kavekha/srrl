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
        rectangle::Rectangle,
    },
    map_builders::pathfinding::Position, SHOW_MAPGEN_VISUALIZER,
};


pub struct BuilderMap {
    pub spawn_list: Vec<(usize, String)>,
    pub map: Map,
    pub starting_position: Option<Position>,
    pub rooms: Option<Vec<Rectangle>>,
    pub history: Vec<Map>
}

impl BuilderMap {
    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let mut snapshot = self.map.clone();
            self.history.push(snapshot);
        }
    }
}

pub struct BuilderChain {
    starter: Option<Box<dyn InitialMapBuilder>>,
    builders: Vec<Box<dyn MetaMapBuilder>>,
    pub build_data: BuilderMap,
}

impl BuilderChain {
    pub fn new() -> BuilderChain {
        BuilderChain{
            starter: None,
            builders: Vec::new(),
            build_data: BuilderMap {
                spawn_list: Vec::new(),
                map: Map::new(),
                starting_position: None,
                rooms: None,
                history: Vec::new(),
            }
        }
    }
    pub fn start_with(&mut self, starter: Box<dyn InitialMapBuilder>) {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder.")
        };
    }
    pub fn with(&mut self, metabuilder: Box<dyn MetaMapBuilder>) {
        self.builders.push(metabuilder);
    }
    pub fn build_map(&mut self) {   // TODO: Add RNG Seed there.
        match &mut self.starter {
            None => panic!("Cannot run a map builder without starting build system"),
            Some(starter) => {
                // Build starter Builder.
                starter.build_map(&mut self.build_data);
            }
        }
        // Build additional layers
        for metabuilder  in self.builders.iter_mut() {
            metabuilder.build_map(&mut self.build_data);
        }
    }
    pub fn spawn_entities(&mut self) {
        let mut entities_pos: Vec<Position> = Vec::new();
        for (i, _room) in self.build_data.spawn_list.iter().enumerate().skip(1){
            //let position = self.rooms[i].center();
            //entities_pos.push(Position(position.0, position.1)); 
            println!("Retourne une entité à spawner."); //TODO
        }
        //entities_pos
    }
    pub fn get_starting_position(&mut self) {
        println!("Retourner une starting position or not?");
    }
}

pub trait InitialMapBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap);
}

pub trait MetaMapBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap);
}


pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self) -> Vec<Position>;
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
    fn get_snapshot_history(&self) -> Vec<Map>;
    fn take_snapshot(&mut self);
}

pub fn random_builder() -> BuilderChain {
    let mut builder = BuilderChain::new();

    builder.start_with(SimpleMapBuilder::new());
    builder.with(RoomBasedSpawner::new());
    builder.with(RoomBasedStartingPosition::new());
    builder.with(RoomBasedStairs::new());
    builder
}

#[derive(Resource)]
pub struct MapGenHistory{
    pub history: Vec<Map>,
    pub index: usize,
}
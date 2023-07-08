pub mod map;
pub mod commons;

mod rectangle;
mod builders;
mod maps; 

pub use commons::TileType;

use crate::{
    map_builders::{
        rectangle::Rectangle,
        builders::{
            room_based_exits::RoomBasedExits,
            room_based_spawner::RoomBasedSpawner,
            room_based_starting_position::RoomBasedStartingPosition,
        },
        maps::sewer_map::SewerMapBuilder,
        map::Map,      
    },
    globals::SHOW_MAPGEN_VISUALIZER, vectors::Vector2Int,
};


#[derive(Clone)]
pub struct BuilderMap {
    pub spawn_list: Vec<Vector2Int>, //Vec<(usize, String)>,
    pub map: Map,
    pub starting_position: Option<Vector2Int>,
    pub rooms: Option<Vec<Rectangle>>,
    pub history: Vec<Map>,
    pub exit_position: Option<Vector2Int>   // TODO ; plusieurs sorties.
}

impl BuilderMap {
    fn take_snapshot(&mut self) {
        if SHOW_MAPGEN_VISUALIZER {
            let snapshot = self.map.clone();
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
                exit_position: None,    //TODO: Plusieurs sorties?
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
    pub fn spawn_entities(&mut self) -> Vec<Vector2Int> {
        let spawn_list = self.build_data.spawn_list.clone();
        spawn_list
    }
    pub fn get_starting_position(&mut self) -> Vector2Int {
        if let Some(starting_position) = self.build_data.starting_position {
            starting_position
        } else {
            panic!("Pas de position de depart")
        }
    }
    pub fn get_exit_position(&mut self) -> Vector2Int {
        if let Some(exit_position) = self.build_data.exit_position {
            exit_position
        } else {
            panic!("Pas de position de depart")
        }
    }
}

pub trait InitialMapBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap);
}

pub trait MetaMapBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap);
}

pub fn random_builder() -> BuilderChain {
    //let mut rng = rand::thread_rng();   //TODO : Seed & refacto.

    let mut builder = BuilderChain::new();
    //builder.start_with(SimpleMapBuilder::new());
    builder.start_with(SewerMapBuilder::new());    

    //let (random_starter, has_rooms) = random_initial_builder(rng);
    //if has_rooms {
        builder.with(RoomBasedSpawner::new());
        builder.with(RoomBasedExits::new());
        builder.with(RoomBasedStartingPosition::new());
    /* 
    } else {
        builder.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
        builder.with(CullUnreachable::new());
        builder.with(VoronoiSpawning::new());
        builder.with(DistantExit::new());
    }
    */

    builder
}

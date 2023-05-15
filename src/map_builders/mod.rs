pub mod map;
pub mod rectangle;
pub mod simple_map;
pub mod pathfinding;
pub mod commons;


use crate::{
    map_builders::{
        map::Map,
        simple_map::SimpleMapBuilder,
    },
    map_builders::pathfinding::Position,
};


trait MapBuilder {
    fn build() -> (Map, Position);
}

pub fn build_random_map() -> (Map, Position) {
    SimpleMapBuilder::build()
}
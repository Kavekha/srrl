use rand::prelude::*;

use super::Map;

use crate::{
    map_builders::{
        MetaMapBuilder,
        rectangle::Rectangle,
        commons::{TileType, apply_room_to_map, apply_horizontal_tunnel, apply_vertical_tunnel},
        pathfinding::Position,
        InitialMapBuilder, BuilderMap,
    },  
    SHOW_MAPGEN_VISUALIZER, 
};

pub struct RoomBasedStartingPosition {}

impl MetaMapBuilder for RoomBasedStartingPosition {
    fn build_map(&mut self, build_data : &mut BuilderMap)  {
        self.build(build_data);
    }
}

impl RoomBasedStartingPosition {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomBasedStartingPosition> {
        Box::new(RoomBasedStartingPosition{})
    }

    fn build(&mut self, build_data : &mut BuilderMap) {
        if let Some(rooms) = &build_data.rooms {
            let start_pos = rooms[0].center();
            build_data.starting_position = Some(Position(start_pos.0, start_pos.1));
        } else {
            panic!("Room Based Staring Position only works after rooms have been created");
        }
    }
}
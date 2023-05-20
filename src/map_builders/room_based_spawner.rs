use rand::prelude::*;

use crate::{
    map_builders::{
        rectangle::Rectangle,
        commons::{TileType, apply_room_to_map, apply_horizontal_tunnel, apply_vertical_tunnel},
        pathfinding::Position,
        InitialMapBuilder, BuilderMap, MetaMapBuilder,
    },  
    SHOW_MAPGEN_VISUALIZER, 
};



pub struct RoomBasedSpawner {}

impl MetaMapBuilder for RoomBasedSpawner {
    fn build_map(&mut self, build_data : &mut BuilderMap)  {
        self.build(build_data);
    }
}

impl RoomBasedSpawner {
    pub fn new() -> Box<RoomBasedSpawner> {
        Box::new(RoomBasedSpawner { })
    }
    fn build(&mut self, build_data: &mut BuilderMap) {
        if let Some(rooms) = &build_data.rooms {
            for (i, _room) in rooms.iter().enumerate().skip(1) {
                let npc_pos = rooms[i].center();
                println!("Position dans npc_pos : {:?}", npc_pos);
                build_data.spawn_list.push(Position(npc_pos.0, npc_pos.1));

                println!("Spawn entities by room!");
            }
        } else {
            panic!("Room Based Spawner MetaBuilder only works after rooms have been created.");
        }
    }
}
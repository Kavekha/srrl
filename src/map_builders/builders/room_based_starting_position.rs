use crate::{
    map_builders::{
        MetaMapBuilder,
        BuilderMap,
    }, vectors::Vector2Int, 
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
        println!("-- RoomBasedStartingPosition: lancement --");
        if let Some(rooms) = &build_data.rooms {
            println!("Il y a des Rooms");
            let start_pos = rooms[0].center();
            build_data.starting_position = Some(Vector2Int{x:start_pos.0, y:start_pos.1});
        } else {
            panic!("Room Based Staring Position only works after rooms have been created");
        }
    }
}
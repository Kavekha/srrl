use crate::{
    map_builders::{
        MetaMapBuilder,
        commons::{TileType},
        BuilderMap,
    },  
    //SHOW_MAPGEN_VISUALIZER, 
};


pub struct RoomBasedExits {}

impl MetaMapBuilder for RoomBasedExits {
    fn build_map(&mut self,build_data : &mut BuilderMap)  {
        self.build(build_data);
    }
}

impl RoomBasedExits {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomBasedExits> {
        Box::new(RoomBasedExits{ })
    }

    fn build(&mut self, build_data : &mut BuilderMap) {
        if let Some(rooms) = &build_data.rooms {
            let exit_position = rooms[rooms.len()-1].center();
            let exit_idx = build_data.map.xy_idx(exit_position.0, exit_position.1);
            build_data.map.tiles[exit_idx] = TileType::Exit;
            build_data.take_snapshot();
        } else {
            panic!("Room Based Exits only works after rooms have been created");
        }
    }
}
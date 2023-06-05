use crate::{
    map_builders::{
         BuilderMap, MetaMapBuilder,
    }, vectors::Vector2Int,  
    //SHOW_MAPGEN_VISUALIZER, 
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
                build_data.spawn_list.push(Vector2Int{x:npc_pos.0, y:npc_pos.1});
                //println!("Check spawn_list : {:?}", build_data.spawn_list);
            }
        } else {
            panic!("Room Based Spawner MetaBuilder only works after rooms have been created.");
        }
    }
}
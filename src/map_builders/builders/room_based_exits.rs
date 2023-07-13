use crate::{
    map_builders::{
        MetaMapBuilder,
        BuilderMap,
    }, vectors::Vector2Int,  
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
            let mut exit_position = rooms[rooms.len()-1].center();

            // A-t-on la position du joueur?
            if let Some(player_starting_position) = &build_data.starting_position {
                let mut farest_distance = 0;
                for room in rooms.iter() {
                    let room_center = room.center();
                    let distance = player_starting_position.clone().manhattan(Vector2Int {x: room_center.0, y: room_center.1 });
                    if distance > farest_distance {
                        farest_distance = distance;
                        exit_position = room.center(); 
                    }
                }
            }

            /*
            let exit_idx = build_data.map.xy_idx(exit_position.0, exit_position.1);
            build_data.map.tiles[exit_idx] = TileType::Exit;
             */
            build_data.exit_position = Some(Vector2Int { x:exit_position.0, y:exit_position.1 });


            build_data.take_snapshot();
        } else {
            panic!("Room Based Exits only works after rooms have been created");
        }
    }
}
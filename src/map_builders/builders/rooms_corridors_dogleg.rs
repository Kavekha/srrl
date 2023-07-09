use rand::Rng;

use crate::map_builders::{MetaMapBuilder, BuilderMap, commons::{apply_horizontal_tunnel, apply_vertical_tunnel}, rectangle::Rectangle};

pub struct DoglegCorridors {
    corridor_size: i32
}

impl MetaMapBuilder for DoglegCorridors {
    #[allow(dead_code)]
    fn build_map(&mut self, build_data : &mut BuilderMap) {
        self.corridors(build_data);
    }
}

impl DoglegCorridors {
    #[allow(dead_code)]
    pub fn new( corridor_size: i32) -> Box<DoglegCorridors> {
        Box::new(DoglegCorridors { corridor_size })
    }

    fn corridors(&mut self, build_data : &mut BuilderMap) {
        let mut rng = rand::thread_rng();   //TODO : Refacto, Seed à plus haut niveau.

        let rooms : Vec<Rectangle>;
        if let Some(rooms_builder) = &build_data.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("Dogleg Corridors require a builder with room structures");
        }

        for (i,room) in rooms.iter().enumerate() {
            if i > 0 {
                let (new_x, new_y) = room.center();
                let (prev_x, prev_y) = rooms[i as usize -1].center();
                if rng.gen_range(0.. 2) == 1 {
                    for size in -1.. self.corridor_size -1 {
                        apply_horizontal_tunnel(&mut build_data.map, prev_x + size, new_x + size, prev_y + size);
                        apply_vertical_tunnel(&mut build_data.map, prev_y + size, new_y + size, new_x + size);
                    }
                } else {
                    for size in -1.. self.corridor_size -1 {                        
                        apply_vertical_tunnel(&mut build_data.map, prev_y + size, new_y + size, new_x + size);
                        apply_horizontal_tunnel(&mut build_data.map, prev_x + size, new_x + size, prev_y + size);
                    }
                }
                build_data.take_snapshot();
            }
        }
    }
}
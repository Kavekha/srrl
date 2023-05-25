use rand::prelude::*;


use crate::{
    map_builders::{
        rectangle::Rectangle,
        commons::{apply_room_to_map, apply_horizontal_tunnel, apply_vertical_tunnel},
        InitialMapBuilder, BuilderMap,
    },  
    //SHOW_MAPGEN_VISUALIZER, 
};


pub struct SimpleMapBuilder {}

impl InitialMapBuilder for SimpleMapBuilder {
    fn build_map(&mut self, build_data: &mut BuilderMap) {
        self.rooms_and_corridors(build_data);
    }
}

impl SimpleMapBuilder {
    pub fn new() -> Box<SimpleMapBuilder> {
        Box::new(SimpleMapBuilder {  })
    }

    fn rooms_and_corridors(&mut self, build_data: &mut BuilderMap) {
        println!("-- SimpleMapBuilder: rooms and corridors --");
        const MAX_ROOMS : i32 = 30;
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 10;
        let mut rooms : Vec<Rectangle> = Vec::new();

        let mut rng = rand::thread_rng();   //TODO : Refacto, Seed à plus haut niveau.

        //println!("Starting: Generate rooms");
        for _i in 0..MAX_ROOMS {
            // generate a room as a Rectangle
            let w = rng.gen_range(MIN_SIZE.. MAX_SIZE);
            let h = rng.gen_range(MIN_SIZE.. MAX_SIZE);

            let x = rng.gen_range(1.. (build_data.map.width - w - 1)) - 1; 
            let y = rng.gen_range(1.. (build_data.map.height - h - 1)) - 1;

            let new_room = Rectangle::new(x, y, w, h);    
            //println!("new room : {},{},{},{}", x, y, w, h);

            // Can I add the room without intersecting with another?
            let mut can_add_room = true;

            for other_room in rooms.iter() {
                if new_room.intersect(other_room) { 
                    can_add_room = false;
                    //println!("Room can't be add there.");
                }
                
            }
            if can_add_room {
                //println!("Room ajoutée à la map");
                apply_room_to_map(&mut build_data.map, &new_room);  
                build_data.take_snapshot(); 

                // Join the new room to the previous one
                if !rooms.is_empty() {
                    //println!("Corridors en cours de creation pour la room");
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = rooms[rooms.len()-1].center();
                    if rng.gen_range(0.. 2) == 1 {
                        apply_horizontal_tunnel(&mut build_data.map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut build_data.map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut build_data.map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut build_data.map, prev_x, new_x, new_y);
                    }
                }     
                rooms.push(new_room);   
                build_data.take_snapshot();         
            }
        }
        build_data.rooms = Some(rooms);
    }
}

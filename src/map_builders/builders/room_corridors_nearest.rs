use std::collections::HashSet;

use crate::{map_builders::{MetaMapBuilder, BuilderMap, rectangle::Rectangle, commons::draw_corridor}, vectors::Vector2Int};

pub struct NearestCorridors {}

impl MetaMapBuilder for NearestCorridors {
    #[allow(dead_code)]
    fn build_map(&mut self, build_data : &mut BuilderMap) {
        self.corridors(build_data);
    }
}

impl NearestCorridors {
    #[allow(dead_code)]
    pub fn new() -> Box<NearestCorridors> {
        Box::new(NearestCorridors{})
    }

    fn corridors(&mut self, build_data : &mut BuilderMap) {
        let rooms : Vec<Rectangle>;
        if let Some(rooms_builder) = &build_data.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("Nearest Corridors require a builder with room structures");
        }

        let mut connected : HashSet<usize> = HashSet::new();
        for (i,room) in rooms.iter().enumerate() {
            let mut room_distance : Vec<(usize, f32)> = Vec::new();
            let room_center = room.center();
            let room_center_pt = Vector2Int{ x: room_center.0, y: room_center.1 };
            for (j,other_room) in rooms.iter().enumerate() {
                if i != j && !connected.contains(&j) {
                    let other_center = other_room.center();
                    let other_center_pt = Vector2Int {x: other_center.0, y: other_center.1 };
                    let distance = room_center_pt.manhattan(other_center_pt);
                    room_distance.push((j, distance as f32));
                }
            }

            if !room_distance.is_empty() {
                room_distance.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap() );
                let dest_center = rooms[room_distance[0].0].center();
                draw_corridor(
                    &mut build_data.map,
                    room_center.0, room_center.1,
                    dest_center.0, dest_center.1
                );
                connected.insert(i);
                build_data.take_snapshot();
            }
        }
    }
}
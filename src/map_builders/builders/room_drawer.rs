use rand::Rng;

use crate::{map_builders::{MetaMapBuilder, BuilderMap, rectangle::Rectangle, TileType}, vectors::Vector2Int};

pub struct RoomDrawer {}

impl MetaMapBuilder for RoomDrawer {
    fn build_map(&mut self, build_data : &mut BuilderMap)  {
        self.build(build_data);
    }
}
 
impl RoomDrawer {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomDrawer> {
        Box::new(RoomDrawer{})
    }

    fn build(&mut self, build_data : &mut BuilderMap) {
        let rooms : Vec<Rectangle>;
        if let Some(rooms_builder) = &build_data.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("Room Rounding require a builder with room structures");
        }

        for room in rooms.iter() {
            let mut rng = rand::thread_rng();
            let room_type = rng.gen_range(0.. 4);
            match room_type {
                1 => self.circle(build_data, room),
                _ => self.rectangle(build_data, room)
            }                       
            build_data.take_snapshot();
        }
    }

    fn rectangle(&mut self, build_data : &mut BuilderMap, room : &Rectangle) {
        for y in room.y1 +1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = build_data.map.xy_idx(x, y);
                if idx > 0 && idx < ((build_data.map.width * build_data.map.height)-1) as usize {
                    build_data.map.tiles[idx] = TileType::Floor;
                }
            }
        }
    }

    fn circle(&mut self, build_data : &mut BuilderMap, room : &Rectangle) {
        let radius = i32::min(room.x2 - room.x1, room.y2 - room.y1) as f32 / 2.0;
        let center = room.center();
        let center_pt = Vector2Int { x:center.0, y:center.1 };
        for y in room.y1 ..= room.y2 {
            for x in room.x1 ..= room.x2 {
                let idx = build_data.map.xy_idx(x, y);
                let distance = center_pt.manhattan(Vector2Int {x: x, y: y }); //rltk::DistanceAlg::Pythagoras.distance2d(center_pt, rltk::Point::new(x, y));
                if idx > 0 
                    && idx < ((build_data.map.width * build_data.map.height)-2) as usize 
                    && distance as f32 <= radius
                {
                    build_data.map.tiles[idx] = TileType::Floor;
                }
            }
        }
    }

}